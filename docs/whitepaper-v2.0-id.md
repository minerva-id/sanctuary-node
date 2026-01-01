# **TESSERAX PROTOCOL**

### **A Mathematical Approach to Post-Quantum Distributed Ledgers**

Version: 2.0  
Date: January 2026  
Authors: Minerva & Gemini (The Architect)
---

**Abstract**

Pengembangan teknologi buku besar terdistribusi (*distributed ledger*) menghadapi tantangan dalam menyeimbangkan prediktabilitas ekonomi dan keamanan data jangka panjang. Tesserax Protocol hadir sebagai infrastruktur Layer-1 yang dibangun di atas kerangka kerja Substrate, dengan fokus pada ketahanan kriptografi pasca-kuantum (*post-quantum resistance*) dan kebijakan moneter deterministik. Protokol ini menggunakan konstanta universal ($\pi, e, \phi$) sebagai dasar perhitungan suplai aset, menghilangkan variabel keputusan manusia dalam tata kelola moneter, serta menyediakan lingkungan eksekusi yang kompatibel dengan standar industri (EVM).

---

**1\. Introduction**

Evolusi aset digital menuntut infrastruktur yang tidak hanya aman saat ini, tetapi juga tangguh terhadap ancaman komputasi masa depan. Tesserax Protocol dirancang sebagai mesin keadaan (*state machine*) yang memprioritaskan dua pilar utama:

1. **Kepastian Matematis:** Menghindari inflasi arbitrer dengan mengikat parameter ekonomi pada konstanta matematika fundamental.  
2. **Keamanan Kuantum:** Mengintegrasikan skema tanda tangan *Lattice-based* untuk melindungi aset pengguna dari potensi dekripsi oleh komputer kuantum.

Tujuan Tesserax adalah menyediakan lapisan dasar (*base layer*) yang stabil, transparan, dan terukur untuk aplikasi terdesentralisasi.

---

## **2\. Economic Architecture: Algorithmic Scarcity**

### **2.1. Derivation of Supply**

Alih-alih memilih angka suplai secara acak, Tesserax menurunkan batas suplainya dari interaksi tiga konstanta matematika utama. Hal ini dilakukan untuk memastikan properti unik dan deterministik pada protokol.

$$S_{max} = \lfloor \pi \times e \times \phi \times 10^6 \rfloor$$

Dengan nilai presisi:

* $\pi \approx 3.14159...$ (Konstanta Siklus)
* $e \approx 2.71828...$ (Konstanta Pertumbuhan)
* $\phi \approx 1.61803...$ (Konstanta Proporsi)

Maka, batas suplai protokol ditetapkan secara permanen pada:

$$\mathbf{13,817,422 \ TSX}$$

### **2.2. Deterministic Emission**

Distribusi token diatur oleh fungsi logistik (Sigmoid) yang telah dikalkulasi sebelumnya (*pre-computed*). Model ini dipilih untuk memberikan distribusi yang terukur: akselerasi pada fase awal jaringan, diikuti oleh perlambatan gradual saat jaringan mencapai maturitas.

$$S(t) = \frac{S_{max}}{1 + e^{-k(t - t_0)}}$$
Dimana $t$ merepresentasikan *Era* jaringan. Pendekatan ini memberikan kepastian bagi partisipan jaringan mengenai tingkat inflasi dan sisa suplai di setiap titik waktu.

---

## 3. Cryptographic Architecture (Security)

### 3.1. Hybrid Dual-Stack Identity

Tesserax mengakui realitas adopsi saat ini (EVM) sambil bersiap untuk ancaman masa depan (Quantum). Oleh karena itu, protokol mendukung dua skema tanda tangan:

1. **Hot Wallet (Legacy):** Menggunakan kurva eliptik `secp256k1` (ECDSA). Ini memungkinkan kompatibilitas 100% dengan Metamask, Remix, dan toolset Ethereum yang ada.
2. **Cold Vault (Quantum):** Menggunakan algoritma **CRYSTALS-Dilithium** (FIPS 204 Standard). Skema ini tahan terhadap serangan algoritma Shor pada komputer kuantum.

### 3.2. The Quantum Vault Mechanism

Fitur unggulan Tesserax adalah **Opt-in Quantum Security**.
Pengguna dapat memanggil fungsi `create_vault()` untuk meningkatkan status akun mereka.

* **Logic:** Aset di dalam Vault dikunci oleh kunci publik Dilithium.
* **Protection:** Meskipun *private key* ECDSA pengguna diretas oleh komputer kuantum di masa depan, aset di dalam Vault tetap aman karena verifikasi transaksi mewajibkan tanda tangan Dilithium yang secara matematis kebal.

---

**4\. Technical Framework**

### **4.1. Consensus Mechanism**

Tesserax menggunakan pendekatan bertahap untuk mekanisme konsensus:

**Fase Testnet (Authority Round):**
* **Aura (Authority Round):** Produksi blok deterministik dengan validator terpilih.
* **GRANDPA:** Finalitas blok yang cepat dan deterministik.

**Fase Mainnet (Nominated Proof of Stake):**
* **BABE (Blind Assignment for Blockchain Extension):** Produksi blok probabilistik dengan VRF.
* **GRANDPA:** Finalitas blok yang deterministik.
* **NPoS:** Mekanisme staking dengan nominasi untuk keamanan ekonomi.

### **4.2. Execution Environment**

Protokol menyediakan kompatibilitas penuh dengan **Ethereum Virtual Machine (EVM)** melalui modul Frontier.

* **Chain ID:** 13817  
* **Compatibility:** Mendukung *smart contract* Solidity dan standar JSON-RPC, memungkinkan pengembang untuk memigrasikan aplikasi tanpa modifikasi kode yang signifikan.

---

**5\. Conclusion**

Tesserax Protocol merepresentasikan pendekatan teknis dan matematis terhadap aset digital. Dengan menggabungkan ekonomi yang diatur oleh konstanta universal dan keamanan kriptografi modern, Tesserax bertujuan untuk menjadi infrastruktur yang andal, aman, dan berumur panjang dalam ekosistem teknologi terdistribusi.