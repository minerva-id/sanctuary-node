# **PROJECT BLUEPRINT: TESSERAX NODE (v2.0)**

## **1. Arsitektur Tingkat Tinggi**

Proyek ini akan dibangun menggunakan **Substrate Framework** (Polkadot SDK) versi stabil terbaru.

* **Node Name:** `tesserax-node`
* **Consensus:** BABE (Block Production) + GRANDPA (Finality).
* **Sybil Resistance:** NPoS (Nominated Proof of Stake).
* **Execution:** EVM (Frontier).
* **Economics:** Hardcoded Sigmoid Emission.
* **Special Feature:** Opt-in Quantum Vault.

---

## **2. Struktur Direktori Proyek**

Struktur repositori Rust akan mengikuti standar *Substrate Node Template* yang telah dibersihkan (*stripped down*).

```text
tesserax-core/
├── node/                   # Logika Node (RPC, CLI, Service, Chain Spec)
│   └── src/
│       ├── chain_spec.rs   # Genesis Config (Hardcoded Constants ada di sini)
│       └── service.rs      # Setup Full Client & Light Client
├── runtime/                # Logika State Machine (The "Brain")
│   └── src/
│       ├── lib.rs          # Menggabungkan semua Pallet
│       └── precompiles.rs  # EVM Precompiles (jika diperlukan nanti)
├── pallets/                # Custom Runtime Modules
│   ├── emission/           # Logika Distribusi Reward (Sigmoid Statis)
│   └── quantum-vault/      # Logika Penyimpanan Dingin PQC (High Fee)
├── scripts/                # Utility Scripts
│   └── generate_curve.py   # Python script untuk menghitung tabel emisi
└── Cargo.toml              # Workspace configuration

```

---

## **3. Komponen Runtime (Pallets)**

Kita membagi modul menjadi dua kategori: **Standard (Off-the-shelf)** dan **Custom (Minimalist)**.

### **A. Pallet Standar (Warisan Polkadot/Frontier)**

Jangan buat ulang roda. Gunakan modul yang sudah diaudit:

1. `frame_system`: Low-level system types.
2. `pallet_balances`: Token logic ($TSRX).
3. `pallet_staking`: Logika NPoS (Validator & Nominator).
4. `pallet_timestamp`: Waktu blok.
5. `pallet_transaction_payment`: Fee logic (dikonfigurasi dengan EIP-1559 multiplier).
6. `pallet_evm` & `pallet_ethereum`: Lapisan kompatibilitas Ethereum penuh.
7. `pallet_base_fee`: Untuk EIP-1559 dynamic base fee.

### **B. Pallet Custom 1: `pallet-emission` (The Economics)**

Ini menggantikan modul ASM yang rumit di v1.0.

* **Fungsi:** Pada setiap blok (`on_initialize`), pallet ini mengambil jumlah token yang harus dicetak dari tabel konstanta, lalu menyetorkannya ke akun `BlockAuthor` (Validator).
* **Storage:** Tidak ada (Stateless).
* **Konstanta (Config):**
```rust
// Array statis berisi reward per blok untuk 100 tahun (dikompresi atau per era)
const EMISSION_TABLE: &[u128]; 

```



### **C. Pallet Custom 2: `pallet-quantum-vault` (The Vault)**

Fitur premium untuk keamanan jangka panjang.

* **Storage:**
* `Vaults(AccountId) -> Option<DilithiumPublicKey>`


* **Calls (Extrinsics):**
1. `create_vault(pqc_key)`:
* Mengubah status akun menjadi "Vault".
* **Biaya:** 10 $TSRX (Sangat mahal untuk mencegah spam).


2. `vault_transfer(signature, to, amount)`:
* Hanya bisa dipanggil jika signature valid diverifikasi oleh `DilithiumPublicKey` yang terdaftar.
* **Biaya:** 100x Base Fee.




* **Hook:**
* Mencegah `pallet_balances::transfer` standar jika akun pengirim terdaftar di `Vaults`.



---

## **4. Genesis & Konfigurasi Ekonomi (Python Script)**

Sebelum menulis Rust, kita harus menghasilkan "DNA Matematika" protokol ini. Jalankan script ini sekali, outputnya di-copy ke `runtime/src/constants.rs`.

**File: `scripts/generate_curve.py**`

```python
import math

# --- KONSTANTA UNIVERSAL ---
PI  = 3.1415926535
E   = 2.7182818284
PHI = 1.6180339887

# Target Supply: ~13,817,580
S_MAX = int(math.floor(PI * E * PHI * 1_000_000)) * (10**18) 

# Durasi Emisi (misal: kurva melandai penuh di 20 tahun)
# 1 Blok = 6 detik. 1 Tahun = 5,256,000 blok.
TOTAL_YEARS = 20
TOTAL_BLOCKS = TOTAL_YEARS * 5_256_000
MIDPOINT = TOTAL_BLOCKS / 2  # Titik infleksi di tahun ke-10

# Growth Rate (k)
# Diset agar S(0) mendekati 0 dan S(end) mendekati S_MAX
k = 10 / MIDPOINT 

print(f"Generating Emission Table for Tesserax Protocol v2.0")
print(f"Max Supply: {S_MAX / 10**18:,.2f} TSRX")

# Kita tidak menyimpan setiap blok (terlalu besar), kita simpan per ERA (24 jam)
# 1 Era = 14,400 blok (asumsi 6s block time)
BLOCKS_PER_ERA = 14_400
TOTAL_ERAS = int(TOTAL_BLOCKS / BLOCKS_PER_ERA)

rewards_per_era = []
prev_supply = 0

for era in range(1, TOTAL_ERAS + 1):
    t = era * BLOCKS_PER_ERA
    
    # Rumus Sigmoid
    current_supply = S_MAX / (1 + math.exp(-k * (t - MIDPOINT)))
    
    # Supply yang boleh dicetak di era ini
    mint_amount = current_supply - prev_supply
    
    # Reward per blok di era ini
    reward_per_block = mint_amount / BLOCKS_PER_ERA
    
    rewards_per_era.append(int(reward_per_block))
    prev_supply = current_supply

# Output format array Rust
print("\n// Copy this into runtime/src/constants.rs")
print(f"pub const REWARD_SCHEDULE: [u128; {len(rewards_per_era)}] = [")
for i, r in enumerate(rewards_per_era):
    if i % 10 == 0: print("    ", end="")
    print(f"{r}, ", end="")
    if (i + 1) % 10 == 0: print() # Newline every 10 items
print("];")

```

---

## **5. Roadmap Pengembangan (Implementation Plan)**

### **Phase 1: Foundation (Minggu 1-2) - ✅ DONE**

* Inisialisasi repo Substrate Node Template.
* Pembersihan pallet bawaan (Aura diganti BABE jika perlu, atau tetap Aura untuk MVP).
* Integrasi `pallet-emission` dengan tabel hasil Python script.
* **Goal:** Chain berjalan, blok diproduksi, reward TSRX tercetak sesuai kurva sigmoid.

### **Phase 2: EVM Layer (Minggu 3-4) - ✅ DONE**

* Integrasi Frontier (`pallet-evm`, `pallet-ethereum`).
* Konfigurasi Chain ID dan Genesis Account (untuk testing Metamask).
* Setup EIP-1559 (Base fee burning).
* **Goal:** Bisa connect Metamask, deploy Smart Contract via Remix.

### **Phase 3: The Quantum Vault (Minggu 5-6) - ✅ DONE**

* Implementasi `pallet-quantum-vault`.
* Integrasi library `pqcrypto` (Rust) atau implementasi *binding* C untuk verifikasi Dilithium/Falcon.
* Testing unit untuk mekanisme penguncian (Locking mechanism).
* **Goal:** Akun bisa di-lock, transaksi biasa gagal, transaksi vault berhasil.

### **Phase 4: Testnet & Hardening (Minggu 7-8) - ✅ DONE**

* Unit testing lengkap (19 tests passing).
* Benchmarking module setup untuk `pallet-quantum-vault`.
* Frontier RPC module integration (`eth.rs`).
* Transfer blocking hooks (`CheckVaultTransfer` TransactionExtension).
* Pembuatan dokumentasi teknis (Whitepaper v2.0).
* **Pending:** Peluncuran Public Testnet.

### **Phase 5: Mainnet (TBD)**

* Security Audit.
* Genesis Block Ceremony.
* Mainnet Launch.

---
