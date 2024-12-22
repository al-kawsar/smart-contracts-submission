# Aplikasi Manajemen Buku

## Deskripsi
Aplikasi ini adalah sistem manajemen buku berbasis teknologi blockchain menggunakan Internet Computer (IC) dengan dukungan stable storage menggunakan struktur data yang stabil (StableBTreeMap). Aplikasi ini menyediakan CRUD (Create, Read, Update, Delete) untuk mengelola buku, seperti menambahkan buku baru, meminjam, mengembalikan, dan menghapus buku.

### Fitur:
- Menambahkan buku baru ke dalam sistem.
- Mengambil buku berdasarkan ID.
- Mendapatkan daftar buku yang tersedia untuk dipinjam.
- Meminjam buku dan mengatur siapa yang meminjam.
- Mengembalikan buku.
- Menghapus buku berdasarkan ID.

### Genre Buku:
- Fiction
- NonFiction
- Science
- Technology

## Instalasi & Menjalankan Aplikasi

### Persyaratan:
- **rustc** versi 1.64 atau lebih tinggi
  ```bash
  $ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
  $ source "$HOME/.cargo/env"
  ```
- **wasm32-unknown-unknown target**:
  ```bash
  $ rustup target add wasm32-unknown-unknown
  ```
- **candid-extractor**:
  ```bash
  $ cargo install candid-extractor
  ```
- **dfx**:
  ```bash
  $ DFX_VERSION=0.15.0 sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
  $ echo 'export PATH="$PATH:$HOME/bin"' >> "$HOME/.bashrc"
  $ source ~/.bashrc
  $ dfx start --background
  ```

### Langkah-Langkah Menjalankan Proyek:

1. **Cloning Repository**:
   ```bash
   git clone https://github.com/al-kawsar/smart-contracts-submission.git
   cd smart-contracts-submission
   ```

2. **Install Dependencies**:
   Pastikan semua dependensi yang dibutuhkan telah diinstal:
   ```bash
   cargo build --release
   ```

Berikut adalah perbaikan untuk format markdown dari bagian **Menjalankan Proyek**:

---

3. **Menjalankan Proyek**:  
Jalankan proyek dengan menggunakan perintah berikut:  
```bash  
npm run gen-deploy  
```  

Jika Anda menghadapi kesalahan izin saat menjalankan `npm run gen-deploy`, ikuti langkah-langkah berikut:  

Jalankan perintah untuk memberikan izin eksekusi:  
```bash  
chmod +x did.sh  
```  

Jalankan kembali perintah deploy:  
```bash  
npm run gen-deploy  
```  

---

Dengan format yang diperbaiki, dokumentasi menjadi lebih mudah dibaca dan mengikuti aturan markdown.

4. **Interaksi dengan API**:  
   Setelah server berjalan, Anda dapat menggunakan perintah berikut untuk mengakses fungsi-fungsi:  
   - `add_book`: Menambahkan buku baru.  
     ```bash  
     dfx canister call icp_rust_boilerplate_backend add_book '(record { title = "Buku Baru"; author = "Penulis"; genre = "Fiction"; })'  
     ```  
   - `get_book`: Mengambil buku berdasarkan ID.  
     ```bash  
     dfx canister call icp_rust_boilerplate_backend get_book '(0)'  
     ```  
   - `update_book`: Memperbarui informasi buku berdasarkan ID.  
     ```bash  
     dfx canister call icp_rust_boilerplate_backend update_book '(0, record { title = "Buku Diperbarui"; author = "Penulis"; genre = "Fiction"; })'  
     ```  
   - `borrow_book`: Meminjam buku.  
     ```bash  
     dfx canister call icp_rust_boilerplate_backend borrow_book '(0)'  
     ```  
   - `return_book`: Mengembalikan buku.  
     ```bash  
     dfx canister call icp_rust_boilerplate_backend return_book '(0)'  
     ```  
   - `delete_book`: Menghapus buku berdasarkan ID.  
     ```bash  
     dfx canister call icp_rust_boilerplate_backend delete_book '(0)'  
     ```  

Dengan perintah-perintah ini, Anda dapat dengan mudah mengelola buku dalam aplikasi blockchain menggunakan canister `icp_rust_boilerplate_backend`.

5. **Mengelola Buku**:
   - Gunakan endpoint-query dan endpoint-update untuk berinteraksi dengan data buku.
