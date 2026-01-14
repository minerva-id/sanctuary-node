#### 1. **Pallet Emission - Dari v1.0 ke v2.0 (Stateless & Pre-Computed)**
   - **Perubahan Utama**
     - Desain philosophy: Dari complex ASM ke simple lookup table (pre-computed off-chain via Python script).
     - Structure: Stateless (no storage, hanya constants seperti `MAX_SUPPLY`, `TOTAL_ERAS`, `BLOCKS_PER_ERA`, `REWARD_SCHEDULE` array).
     - Hooks (`on_initialize`): Hitung era dari block number, lookup reward dari table, mint ke author via `deposit_creating`. Fix bug find author (ganti default digests ke real `frame_system::Pallet::<T>::digests()`).
     - Events: `RewardMinted` (per block) dan `EmissionEnded` (sekali per era setelah all).
     - Errors: `NoAuthor` dan `Overflow`.
     - Helpers: `current_era`, `reward_for_era`, `max_supply`, `total_eras`, `is_emission_ended`, `total_emitted` (sum complete eras + partial).
   - **Alasan**: Deterministic (no floating-point on-chain), auditable (table verifiable via script), efisien (O(1) lookup), no dust fractional.
   - **Code Snippet Kunci**
     ```rust
     fn on_initialize(block_number: BlockNumberFor<T>) -> Weight {
         let block_num: u32 = block_number.try_into().unwrap_or(0);
         let current_era = block_num.saturating_sub(1) / BLOCKS_PER_ERA;
         if current_era >= TOTAL_ERAS as u32 {
             if block_num % BLOCKS_PER_ERA == 1 { Self::deposit_event(Event::EmissionEnded { ... }); }
             return T::WeightInfo::on_initialize_no_reward();
         }
         let reward_per_block = REWARD_SCHEDULE[current_era as usize];
         if reward_per_block == 0 { return T::WeightInfo::on_initialize_no_reward(); }
         let reward: BalanceOf<T> = reward_per_block.try_into().ok_or(Error::<T>::Overflow)?;
         let digests = frame_system::Pallet::<T>::digests();
         let author = T::FindAuthor::find_author(digests.iter()).ok_or(Error::<T>::NoAuthor)?;
         let imbalance = T::Currency::deposit_creating(&author, reward);
         Self::deposit_event(Event::RewardMinted { ... });
         drop(imbalance);
         T::WeightInfo::on_initialize_with_reward()
     }
     ```
   - **Implikasi**: Emission sigmoid (growth early, peak year 10, scarcity late) sekarang solid untuk testnet Aura. Siap migrasi ke NPoS mainnet (payout via staking).

#### 2. **Total Supply - Optimisasi Constants & Coverage**
   - **Perubahan Utama** (dari Python script & hitung ulang):
     - Asli (truncated constants PI=3.1415926535, E=2.7182818284, PHI=1.6180339887): Floor = **13,817,422 TSRX** (S_MAX_UNITS), coverage ~99.9955%.
     - Hitung akurat (math.pi, math.e, exact PHI): Raw ≈13,817,580.227 → floor **13,817,580 TSRX**, emitted ~13,816,952.71 TSRX (coverage 99.99546%).
     - Sisa untuk 100%: ~627 TSRX (bukan 158 — itu dari difference truncated vs akurat).
     - Tambah **bonus mint ~627 TSRX** di akhir (post era 7,300): Mint sekali ke treasury/author setelah emission end, dengan flag untuk prevent repeat.
   - **Alasan**: Truncated constants untuk reproducibility (no drift), tapi akurat lebih "pure". Flooring cumulatif bikin emitted kurang — bonus mint fix tanpa ubah table/curve.
   - **Code Snippet untuk Bonus** (tambah di pallet):
     ```rust
     if current_era >= TOTAL_ERAS as u32 {
         // Existing EmissionEnded...
         if !BonusMinted::<T>::get() {  // Tambah storage bool flag
             let bonus = 627_u128 * 10u128.pow(18);  // Sisa di planck
             let author = T::FindAuthor::find_author(...).unwrap_or_default();  // Atau treasury
             T::Currency::deposit_creating(&author, bonus);
             Self::deposit_event(Event::BonusMinted { amount: 627 });
             BonusMinted::<T>::put(true);
         }
         return T::WeightInfo::on_initialize_no_reward();
     }
     ```
   - **Implikasi**: Supply asymptotic exact 13,817,580 TSRX (whitepaper update ke ini). Keep mathematical (π × e × φ), no dust, auditable.

#### 3. **Quantum Vault Fee - Adjust Realistis**
   - **Perubahan Utama**:
     - Creation: Dari 10 TSRX ke **2 TSRX**.
     - Transfer: Multiplier dari 100x ke **10x** + optional fixed 0.1-0.2 TSRX.
   - **Alasan**: Dengan supply ~13.82M, fee lama terlalu mahal di market cap high. Adjust untuk adoption (retail affordable), tapi compensate node heavy verify.
   - **Implikasi**: Vault lebih usable, align supply kecil. Future: Re-ML batching drop fee via STARK proof.