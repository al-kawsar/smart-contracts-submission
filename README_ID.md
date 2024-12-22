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
        curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
        source "$HOME/.cargo/env"
        ```
        
    - **wasm32-unknown-unknown target**:
        
        ```bash
        rustup target add wasm32-unknown-unknown
        ```
        
    - **candid-extractor**:
        
        ```bash
        cargo install candid-extractor
        ```
        
    - **dfx**:
        
        ```bash
        DFX_VERSION=0.15.0 sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
        echo 'export PATH="$PATH:$HOME/bin"' >> "$HOME/.bashrc"
        source ~/.bashrc
        dfx start --background
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

4. **📱 Interaksi dengan API**: Dengan server yang berjalan:
    ```bash
    http://127.0.0.1:4943/?canisterId=bw4dl-smaaa-aaaaa-qaacq-cai&id=br5f7-7uaaa-aaaaa-qaaca-cai
    ```
    
