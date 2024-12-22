# Aplikasi Manajemen Buku

## 🎯 Deskripsi

Aplikasi ini adalah sistem manajemen buku berbasis teknologi blockchain menggunakan Internet Computer (IC) dengan dukungan stable storage menggunakan struktur data yang stabil (**StableBTreeMap**). Aplikasi ini menyediakan CRUD (Create, Read, Update, Delete) untuk mengelola buku, seperti menambahkan buku baru, meminjam, mengembalikan, dan menghapus buku.

### 🚀 Fitur:

- **📚 Tambahkan buku baru** ke dalam sistem.
- **🔍 Ambil buku berdasarkan ID**.
- **📜 Daftar buku** yang tersedia untuk dipinjam.
- **📖 Pinjam buku** dan atur siapa yang meminjam.
- **↩️ Kembalikan buku**.
- **❌ Hapus buku** berdasarkan ID.

### 🌟 Genre Buku:

- **Fiction**
- **NonFiction**
- **Science**
- **Technology**

---

## 🔧 Langkah-Langkah Menjalankan Proyek:

1. **Cloning Repository**:
    
    ```bash
    git clone https://github.com/al-kawsar/smart-contracts-submission.git
    cd smart-contracts-submission
    ```
    
2. **Install**: Pastikan Anda memiliki lingkungan yang sesuai untuk menjalankan proyek:
    
    - **rustc** versi 1.64 atau lebih tinggi:
        
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
        
3. **Menjalankan Proyek**: Setelah semua komponen diinstal, jalankan aplikasi:
    
    ```bash
    npm run gen-deploy  
    ```
    
    Jika Anda menghadapi masalah izin, pastikan untuk menjalankan:
    
    ```bash
    chmod +x did.sh  
    npm run gen-deploy  
    ```
    

---

4. **📱 Interaksi dengan API**: Dengan server yang berjalan, Anda dapat dengan mudah berinteraksi menggunakan berbagai fungsi:
    
    - **➕ Tambah Buku**:
        
        ```bash
        dfx canister call icp_rust_boilerplate_backend add_book '(record { title = "Buku Baru"; author = "Penulis"; genre = "Fiction"; })'  
        ```
        
    - **🔍 Lihat Buku**:
        
        ```bash
        dfx canister call icp_rust_boilerplate_backend get_book '(0)'  
        ```
        
    - **🔄 Perbarui Buku**:
        
        ```bash
        dfx canister call icp_rust_boilerplate_backend update_book '(0, record { title = "Buku Diperbarui"; author = "Penulis"; genre = "Fiction"; })'  
        ```
        
    - **📚 Pinjam Buku**:
        
        ```bash
        dfx canister call icp_rust_boilerplate_backend borrow_book '(0)'  
        ```
        
    - **↩️ Kembalikan Buku**:
        
        ```bash
        dfx canister call icp_rust_boilerplate_backend return_book '(0)'  
        ```
        
    - **❌ Hapus Buku**:
        
        ```bash
        dfx canister call icp_rust_boilerplate_backend delete_book '(0)'  
        ```
        

Dengan langkah-langkah ini, Anda dapat mengelola koleksi buku secara efektif dan menarik menggunakan teknologi blockchain mutakhir.

5. **📚 Pengelolaan Buku**: Gunakan endpoint-query dan endpoint-update untuk memanage buku dengan fleksibilitas penuh.
