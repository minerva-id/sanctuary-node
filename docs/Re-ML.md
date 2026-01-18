Ini adalah penjelasan mendalam mengenai arsitektur **Recursive-STARK ML-DSA (Re-ML)**.

Penting untuk dipahami bahwa ini **bukanlah sebuah penemuan matematika baru**, melainkan sebuah **arsitektur sistem** (protokol) yang menggabungkan dua standar kriptografi yang sudah ada: **ML-DSA** (untuk tanda tangan pengguna) dan **Recursive ZK-STARKs** (untuk kompresi dan verifikasi data).

Riset akademis nyata yang mendekati konsep ini telah dipublikasikan dengan nama seperti **zk-DASTARK**, yang menggabungkan skema tanda tangan Dilithium (ML-DSA) dengan pembuktian STARK untuk efisiensi dan privasi.

Berikut adalah bedah detail cara kerjanya:

### 1. Masalah Inti: "Bloat" Data

Dalam standar ML-DSA (FIPS 204), satu tanda tangan berukuran sekitar **2.4 KB**.

* Jika sebuah blok Ethereum berisi 1.000 transaksi, maka `1.000 x 2.4 KB = 2.4 MB` hanya untuk tanda tangan saja. Ini akan memacetkan jaringan.

### 2. Mekanisme Kerja Re-ML

Sistem ini bekerja dalam tiga lapisan (*layers*):

#### Lapisan 1: Tanda Tangan Pengguna (User Layer)

Pengguna (Wallet) tidak perlu melakukan sesuatu yang rumit. Mereka tetap menggunakan **ML-DSA** standar.

* **Proses:** Pengguna menandatangani transaksi  menggunakan kunci privat ML-DSA mereka.
* **Keuntungan:** Karena ML-DSA adalah standar NIST, di masa depan *hardware wallet* dan *chip* keamanan (Secure Enclave) di HP akan mendukungnya secara *native*. Pengguna mendapatkan kecepatan penandatanganan yang tinggi tanpa beban komputasi berat di sisi klien.

#### Lapisan 2: Aggregator & Sirkuit Aritmatika (Prover Layer)

Ini adalah tempat "keajaiban" efisiensi terjadi. Transaksi tidak langsung dikirim ke *blockchain* utama (L1), melainkan ke node khusus (Aggregator/Sequencer).

Node ini menjalankan **Sirkuit ZK-STARK**. Sirkuit ini adalah program yang ditulis dalam bahasa ZK (seperti Cairo atau menggunakan zkVM seperti SP1/RISC0) yang melakukan tugas berikut:

1. **Input:** Menerima 1.000 pasang (Pesan + Tanda Tangan ML-DSA + Kunci Publik).
2. **Komputasi:** Di dalam sirkuit tertutup, program memverifikasi matematika ML-DSA (operasi matriks kisi/lattice). Program mengecek: *"Apakah tanda tangan 2.4KB ini valid untuk pesan ini?"*
3. **Output:** Jika valid, sirkuit tidak mengeluarkan tanda tangan asli. Ia mengeluarkan sebuah **Bukti STARK (Proof)** yang secara matematis menjamin bahwa *"Saya telah memeriksa 1.000 tanda tangan ML-DSA dan semuanya valid."*

Tantangan teknis terbesar di sini adalah memprogram verifikasi ML-DSA (yang berbasis *lattice* dan *hashing* SHAKE) ke dalam sirkuit aritmatika ZK, yang bisa memakan biaya komputasi besar. Namun, pustaka seperti *sp1-ntt-gadget* sedang dikembangkan untuk memverifikasi ML-DSA di dalam zkVM.

#### Lapisan 3: Rekursi (The Recursive Part)

Mengapa disebut **Recursive**?
Bayangkan jika ada 100.000 transaksi. Membuat satu bukti raksasa untuk semuanya akan membutuhkan RAM superkomputer yang tidak praktis.
Solusinya adalah **Recursive Proof Composition** (seperti yang dijelaskan oleh Vitalik Buterin dan digunakan dalam zkRollups):

1. **Level 1:** Bagi 100.000 transaksi menjadi 1.000 *batch* (masing-masing 100 transaksi). Hasilkan 1.000 bukti STARK kecil.
2. **Level 2:** Ambil 1.000 bukti tersebut, lalu buat sirkuit baru yang memverifikasi bukti-bukti tersebut. Hasilkan 10 bukti "Master".
3. **Root:** Gabungkan 10 bukti master menjadi **Satu Bukti Akar (Root Proof)**.

Bukti Akar ini ukurannya tetap kecil (misalnya ~40-100 KB), tidak peduli apakah ia mewakili 100 transaksi atau 1 juta transaksi. Inilah yang disebut efisiensi logaritmik (*logarithmic compression*).

### 3. Perbandingan Arsitektur

| Fitur | ML-DSA Murni (Raw) | Recursive-STARK ML-DSA (Re-ML) |
| --- | --- | --- |
| **Ukuran On-Chain (1000 Tx)** | ~2.400 KB (Sangat Berat) | ~100 KB (Sangat Ringan) |
| **Biaya Gas (User)** | Mahal (per byte data) | Murah (biaya verifikasi dibagi rata) |
| **Beban Komputasi Server** | Rendah (Verifikasi cepat) | **Sangat Tinggi** (Membuat bukti ZK butuh server kuat) |
| **Latensi** | Instan (Real-time) | Menengah (Butuh waktu untuk *generate proof*) |
| **Keamanan** | Post-Quantum (Lattice) | Post-Quantum (Lattice + Hash-based STARK) |

### 4. Validasi Riset & Industri

Konsep ini didukung oleh perkembangan terkini:

1. **zk-DASTARK:** Sebuah makalah tahun 2025 mengusulkan skema persis seperti ini untuk *smart contract* data feed. Mereka menggunakan *CRYSTALS-Dilithium* (ML-DSA) untuk tanda tangan dan *zk-STARK* untuk mengompresi verifikasinya agar efisien di blockchain IOTA.
2. **Ethereum Roadmap:** Vitalik Buterin secara eksplisit menyebutkan bahwa strategi terbaik untuk *quantum-proofing* Ethereum adalah dengan melakukan *signature aggregation* menggunakan **Recursive STARKs**, karena BLS (agregasi saat ini) tidak aman terhadap kuantum.
3. **Implementasi zkVM:** Proyek seperti **Succinct Labs (SP1)** dan **Risc0** memungkinkan kode verifikasi ML-DSA (ditulis dalam Rust/C++) dijalankan di dalam lingkungan ZK untuk menghasilkan bukti validitas.

### Kesimpulan

**Recursive-STARK ML-DSA (Re-ML)** adalah solusi jembatan yang pragmatis. Ia memungkinkan kita menggunakan standar keamanan pemerintah (NIST ML-DSA) hari ini, tanpa menghancurkan skalabilitas blockchain, dengan cara "membungkus" data yang berat itu ke dalam bukti matematika yang ringkas.

---

## 5. Implementation Status (Tesserax v3.0)

### ✅ Implemented Components

| Component | Location | Status |
|-----------|----------|--------|
| **Guest (zkVM Circuit)** | `reml/guest/src/main.rs` | ✅ Full ML-DSA Verification (FIPS 204) |
| **Host (Prover)** | `reml/host/src/main.rs` | ✅ Full CLI + SP1 Integration + Server |
| **Shared Types** | `reml/lib/src/lib.rs` | ✅ Complete with Merkle Tree |
| **Verifier Pallet** | `pallets/reml-verifier/src/lib.rs` | ✅ Full Verification + Replay Prevention |
| **Runtime Integration** | `runtime/src/configs/mod.rs` | ✅ Pallet Registered (Index 16) |
| **Weights** | `pallets/reml-verifier/src/weights.rs` | ✅ Realistic Calculations |

### 🔐 Security Features

1. **VKey Binding** - Proofs tied to specific SP1 program version
2. **Merkle Root Verification** - Request IDs committed in proof
3. **Replay Prevention** - Proof commitments tracked on-chain
4. **Aggregator Authorization** - Only registered accounts can submit

### 📁 Code Structure

```
tesserax-node/
├── reml/                          # Re-ML Prover System
│   ├── Cargo.toml                 # Workspace config
│   ├── README.md                  # Documentation
│   ├── lib/                       # Shared types
│   │   └── src/lib.rs             # SignatureRequest, ProofBundle, Merkle
│   ├── guest/                     # zkVM program (SP1)
│   │   └── src/main.rs            # Full ML-DSA verification (NTT, SHAKE256)
│   └── host/                      # Prover CLI
│       └── src/main.rs            # Proof generation, test data, HTTP server
│
├── pallets/
│   └── reml-verifier/             # On-chain verifier
│       ├── src/lib.rs             # Proof verification, aggregator registry
│       └── src/weights.rs         # Benchmark weights
│
└── runtime/src/
    ├── lib.rs                     # RemlVerifier @ pallet_index(16)
    └── configs/mod.rs             # ExpectedVKeyHash configuration
```

### 🚀 Quick Start

```bash
# Install SP1 toolchain
curl -L https://sp1.succinct.xyz | bash && sp1up

# Build Re-ML
cd reml && cargo build --release

# Generate test signatures (real ML-DSA)
cargo run --bin reml-prover -- gen-test -c 10 -o test.json

# Generate STARK proof
cargo run --bin reml-prover -- prove -i test.json -o proof.json

# Verify locally
cargo run --bin reml-prover -- verify -p proof.json

# Get VKey hash for production
cargo run --bin reml-prover -- vkey-hash

# Run aggregator server
cargo run --bin reml-prover -- serve --port 8080
```

### 🔧 Production Deployment

1. Build guest program with SP1 toolchain
2. Extract VKey hash using `vkey-hash` command
3. Update `ExpectedVKeyHash` in `runtime/src/configs/mod.rs`
4. Rebuild runtime and deploy
5. Register aggregator accounts via sudo