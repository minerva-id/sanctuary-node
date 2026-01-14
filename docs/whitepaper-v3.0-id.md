# **TESSERAX PROTOCOL**

### **A Mathematical Approach to Scalable Post-Quantum Distributed Ledgers**

Version: 3.0  
Date: January 2026  
Authors: Minerva & Gemini (The Architect)

---

**Abstract**

Pengembangan teknologi *distributed ledger* menghadapi tantangan dalam menyeimbangkan prediktabilitas ekonomi dan keamanan data jangka panjang. Tesserax Protocol hadir sebagai infrastruktur Layer-1 yang dibangun di atas kerangka kerja Substrate, berfokus pada resistensi kriptografi pasca-kuantum dan kebijakan moneter deterministik.

Protokol ini memelopori arsitektur **Recursive-STARK ML-DSA (Re-ML)**, sebuah solusi novel untuk mengatasi masalah ukuran data (*data bloat*) pada kriptografi berbasis *Lattice*. Dengan mengompresi tanda tangan pasca-kuantum yang berat menjadi bukti *Zero-Knowledge* yang ringkas, Tesserax memastikan keamanan jangka panjang tanpa mengorbankan *throughput* jaringan. Protokol ini tetap menggunakan konstanta universal ($\pi, e, \phi$) sebagai dasar perhitungan suplai aset, menghilangkan variabel keputusan manusia dalam tata kelola moneter.

---

**1. Introduction**

Evolusi aset digital menuntut infrastruktur yang tidak hanya aman saat ini, tetapi juga tangguh terhadap ancaman komputasi masa depan. Tesserax Protocol didesain sebagai *state machine* yang memprioritaskan tiga pilar utama:

1. **Mathematical Certainty:** Menghindari inflasi sewenang-wenang dengan mengikat parameter ekonomi pada konstanta matematika fundamental.

2. **Quantum Security:** Mengintegrasikan skema tanda tangan berbasis *Lattice* (ML-DSA) untuk melindungi aset pengguna dari dekripsi oleh komputer kuantum.

3. **Scalable Integrity:** Menggunakan *recursive proof composition* untuk menggabungkan verifikasi kriptografi yang kompleks menjadi satu bukti kecil, meminimalkan jejak data *on-chain* secara drastis.

Tujuannya adalah menyediakan lapisan dasar yang stabil, transparan, dan terukur untuk aplikasi terdesentralisasi.

---

**2. Economic Architecture: Algorithmic Scarcity**

### **2.1. Derivation of Supply**

Alih-alih memilih angka suplai secara acak, Tesserax menurunkan batas suplainya dari interaksi tiga konstanta matematika utama. Hal ini dilakukan untuk memastikan properti yang unik dan deterministik dalam protokol.

$$S_{max} = \lfloor \pi \times e \times \phi \times 10^6 \rfloor$$

Dengan nilai presisi:

* $\pi \approx 3.14159...$ (Cycle Constant)
* $e \approx 2.71828...$ (Growth Constant)
* $\phi \approx 1.61803...$ (Proportion Constant)

Dengan demikian, batas suplai protokol ditetapkan secara permanen pada:

**13,817,422 TSRX**

### **2.2. Deterministic Emission**

Distribusi token diatur oleh fungsi logistik (Sigmoid) yang telah dihitung sebelumnya. Model ini dipilih untuk memberikan distribusi terukur: akselerasi pada fase awal jaringan, diikuti oleh deselerasi bertahap saat jaringan mencapai kematangan.

$$S(t) = \frac{S_{max}}{1+e^{-k(t-t_0)}}$$

Di mana $t$ mewakili *Era* jaringan. Pendekatan ini memberikan kepastian bagi partisipan jaringan mengenai tingkat inflasi dan sisa suplai di setiap titik waktu.

---

**3. Cryptographic Specifications**

### **3.1. Dual-Stack Identity System (Re-ML Integration)**

Untuk mendukung interoperabilitas dengan infrastruktur yang ada sekaligus meningkatkan keamanan dan skalabilitas, Tesserax menerapkan standar ganda:

1. **Standard Addressing (Secp256k1):** Mendukung alamat standar industri (kompatibel dengan Ethereum) untuk integrasi mudah dengan dompet dan alat pengembangan saat ini.

2. **Quantum Vault (Re-ML Architecture):** Implementasi tingkat lanjut dari algoritma *Module-Lattice-Based Digital Signature* (ML-DSA / FIPS 204).  
   * Alih-alih membebani rantai blok dengan tanda tangan mentah berukuran ~2.4KB, Tesserax menggunakan lapisan **Recursive STARK**.  
   * Tanda tangan pengguna diproses secara *off-chain* untuk menghasilkan bukti validitas matematis. Ini memungkinkan pengguna mengamankan aset di bawah skema enkripsi yang tahan terhadap serangan algoritma Shor, namun dengan biaya gas dan ukuran data yang setara dengan sistem pra-kuantum.

### **3.2. Verification Logic**

Validasi transaksi dalam protokol memprioritaskan bukti kriptografi terkuat yang terkait dengan akun.

* **Proof-Based Validation:** Jika fitur Quantum Vault diaktifkan, protokol tidak memverifikasi tanda tangan satu per satu. Sebaliknya, protokol memverifikasi **Zero-Knowledge Root Proof** yang diajukan oleh *Aggregator*.  
* **Safety Guarantee:** Protokol secara otomatis menolak transaksi yang hanya ditandatangani kunci kurva eliptik standar jika Vault aktif, memastikan integritas aset tetap terlindungi sepenuhnya di bawah payung keamanan pasca-kuantum.

---

**4. Technical Framework**

### **4.1. Consensus Mechanism**

Tesserax menggunakan pendekatan bertahap untuk mekanisme konsensus:

**Testnet Phase (Authority Round):**

* **Aura (Authority Round):** Produksi blok deterministik dengan validator terpilih.

* **GRANDPA:** Finalitas blok yang cepat dan deterministik.

**Mainnet Phase (Nominated Proof of Stake):**

* **BABE (Blind Assignment for Blockchain Extension):** Produksi blok probabilistik dengan VRF.

* **GRANDPA:** Finalitas blok deterministik.

* **NPoS:** Mekanisme staking dengan nominasi untuk keamanan ekonomi.

### **4.2. Off-Chain Aggregation (Prover Network)**

Untuk mendukung arsitektur Re-ML, Tesserax memperkenalkan lapisan agregasi:

* **Batching:** Node Aggregator mengumpulkan transaksi ML-DSA dan membuat bukti STARK tunggal untuk ribuan transaksi.  
* **Recursion:** Bukti-bukti ini dikompresi lebih lanjut secara rekursif hingga menjadi satu *Root Proof* yang diverifikasi oleh Validator Mainnet.

### **4.3. Execution Environment**

Protokol menyediakan kompatibilitas penuh dengan **Ethereum Virtual Machine (EVM)** melalui modul Frontier.

* **Chain ID:** 13817

* **Compatibility:** Mendukung *smart contracts* Solidity dan standar JSON-RPC.

* **ZK-Coprocessor Precompiles:** Lingkungan EVM dilengkapi dengan *precompiled contracts* yang dioptimalkan untuk memverifikasi bukti STARK, memungkinkan pengembang membangun dApps privasi dan keamanan tinggi dengan mudah.

---

**5. Conclusion**

Tesserax Protocol merepresentasikan pendekatan teknis dan matematis terhadap aset digital. Dengan menggabungkan ekonomi yang diatur oleh konstanta universal, keamanan kriptografi modern, dan kini arsitektur **Recursive-STARK** untuk skalabilitas tak terbatas, Tesserax bertujuan menjadi infrastruktur yang andal, aman, dan berumur panjang dalam ekosistem teknologi terdistribusi.