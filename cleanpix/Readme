**Rust-based Image Duplicate Finder**

## 💡 How It Works — Step by Step

### 1. **Scan a Folder Recursively**

- Use Rust’s `std::fs` or a crate like `walkdir` to go through all files in a directory and its subfolders.
- Filter out non-image files based on extension (`.jpg`, `.png`, `.webp`, etc.).

### 2. **Hash Each Image**

There are two approaches:

#### ✅ **A. Byte-level Hash (Simple & Fast)**

- Read the raw image bytes and generate a hash (e.g., SHA256 or MD5).
- Compare hashes — identical hash = duplicate file.

#### 🧠 **B. Perceptual Hash (Better)**

- Use a crate like `img_hash` or `img-pHash`.
- These detect _visually similar_ images (even if resized or compressed).
- Compute hashes and then measure Hamming distance (small distance → likely duplicate).

### 3. **Store and Compare Hashes**

- Maintain a `HashMap<String, Vec<PathBuf>>` where:

  - Key = hash
  - Value = list of files with that hash

- If more than one file has the same hash → report as duplicates.

### 4. **Present Duplicates**

- Print list to terminal, save to file, or show GUI.
- Optionally allow:

  - Deleting duplicates
  - Moving them to a "duplicates" folder
  - Keeping the largest or most recent copy

### 5. **Bonus Features You Can Add**

| Feature                  | Description                                                                 |
| ------------------------ | --------------------------------------------------------------------------- |
| ✅ CLI with `clap`       | Let user pass folder path and flags (e.g., `--delete`, `--hash=perceptual`) |
| 🖼️ Image preview         | Use terminal graphics libs or a GUI like Tauri/egui                         |
| 📁 Export result         | JSON or text report of duplicate groups                                     |
| 🔄 Async file processing | Use threads or async for large folders                                      |
| 📦 Cache hashes          | Skip re-processing files that haven't changed                               |

---

## 🚀 Learning You'll Gain

- Working with `PathBuf`, `std::fs`, and file I/O
- Hashing algorithms
- Using external crates
- Designing clean CLI UX
- (Optionally) multithreading and image processing
