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
   git clone https://github.com/your-repo/manajemen-buku.git
   cd manajemen-buku
   ```

2. **Install Dependencies**:
   Pastikan semua dependensi yang dibutuhkan telah diinstal:
   ```bash
   cargo build --release
   ```

3. **Menjalankan Proyek**:
   Jalankan proyek dengan menggunakan perintah berikut:
   ```bash
   ic-cdk serve
   ```

4. **Interaksi dengan API**:
   Setelah server berjalan, Anda dapat menggunakan perintah berikut untuk mengakses fungsi-fungsi:
   - `add_book`: Menambahkan buku baru.
     ```bash
     ic-cdk-query --query add_book "title: 'Buku Baru', author: 'Penulis', genre: 'Fiction'"
     ```
   - `get_book`: Mengambil buku berdasarkan ID.
     ```bash
     ic-cdk-query --query get_book 1
     ```
   - `borrow_book`: Meminjam buku.
     ```bash
     ic-cdk-update --update borrow_book 1
     ```
   - `return_book`: Mengembalikan buku.
     ```bash
     ic-cdk-update --update return_book 1
     ```
   - `delete_book`: Menghapus buku berdasarkan ID.
     ```bash
     ic-cdk-update --update delete_book 1
     ```

5. **Mengelola Buku**:
   - Gunakan endpoint-query dan endpoint-update untuk berinteraksi dengan data buku.
