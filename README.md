## Book Management Application

### 🎯 Description

This application is a blockchain-based book management system utilizing the Internet Computer (IC) with stable storage support using a robust data structure (**StableBTreeMap**). It provides CRUD (Create, Read, Update, Delete) functionalities for managing books, including adding new books, borrowing, returning, and deleting books.

### 🚀 Features:

- **📚 Add new books** to the system.
- **🔍 Retrieve books by ID**.
- **📜 List available books** for borrowing.
- **📖 Borrow books** and manage who borrows them.
- **↩️ Return books**.
- **❌ Delete books** by ID.

### 🌟 Book Genres:

- **Fiction**
- **NonFiction**
- **Science**
- **Technology**

---

## 🔧 Steps to Run the Project:

1. **Cloning the Repository**:

    ```bash
    git clone https://github.com/al-kawsar/smart-contracts-submission.git
    cd smart-contracts-submission
    ```

2. **Installation**: Ensure you have the necessary environment to run the project:

    - **rustc** version 1.64 or higher:

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

3. **Running the Project**: After all components are installed, run the application:

    ```bash
    npm run gen-deploy  
    ```

    If you encounter permission issues, ensure to run:

    ```bash
    chmod +x did.sh  
    npm run gen-deploy  
    ```

---

4. **📱 Interacting with API**: With the server running:

    ```bash
    http://127.0.0.1:4943/?canisterId=bw4dl-smaaa-aaaaa-qaacq-cai&id=br5f7-7uaaa-aaaaa-qaaca-cai
    ```
