# 🔐 Encrypt Decrypt Tool

A versatile and secure command-line utility for encrypting and decrypting files of various formats using the **Fernet** symmetric encryption algorithm.

## ✨ Key Features

- **🔒 Strong Encryption & Decryption**  
  Securely encrypt and decrypt any file with the powerful **Fernet** encryption algorithm.

- **📂 Multi-format Support**  
  Works with a wide range of file types — including `.mp3`, `.jpeg`, `.png`, `.txt`, and more.

- **🛠 Command-Line Interface**  
  Simple, script-friendly CLI design for quick use and seamless automation in your workflows.

- **📁 Flexible I/O Options**  
  Easily specify input and output paths to maintain full control over where your files go.

---

## ⚙️ How It Works

This tool uses [Fernet encryption](https://cryptography.io/en/latest/fernet/) provided by the `cryptography` Python package. Fernet guarantees that a message encrypted using it cannot be manipulated or read without the key.

---

## 🚀 Getting Started

### 🔧 Prerequisites

Make sure you have Python installed. Then, install the required package:

```bash
pip install cryptography
```

---

## 🧑‍💻 Usage

### 🔐 Encrypt a file
```bash
python encrypt_decrypt_tool.py encrypt input_file [--output_file OUTPUT_FILE]
```

### 🔓 Decrypt a file
```bash
python encrypt_decrypt_tool.py decrypt input_file [--output_file OUTPUT_FILE]
```

### 📌 Example
```bash
python encrypt_decrypt_tool.py encrypt secret.txt --output_file secret.encrypted
python encrypt_decrypt_tool.py decrypt secret.encrypted --output_file secret_decrypted.txt
```

---

## 📽 Preview

[![Encrypt Decrypt Tool Preview]
(https://github.com/4-krishna/File-encryption-decryption-tool/assets/168671541/6a208c39-d56c-4ef8-aa74-29ebf5bf864a)](https://github.com/4-krishna/File-encryption-decryption-tool/assets/168671541/6a208c39-d56c-4ef8-aa74-29ebf5bf864a)

---

## 📁 Supported File Formats

- Text files (`.txt`)
- Audio files (`.mp3`)
- Image files (`.jpeg`, `.png`)
- And virtually any other file type

---

## 🔐 Why Fernet?

Fernet is part of the Python `cryptography` library and ensures:

- Secure symmetric encryption (AES in CBC mode with a SHA256 HMAC)
- Time-based token validity (optional)
- Easy key management and sharing

---

## 📄 License

This project is licensed under the [MIT License](LICENSE).

---

Built with ❤️ using [Python](https://www.python.org/) and [cryptography](https://cryptography.io/).  
