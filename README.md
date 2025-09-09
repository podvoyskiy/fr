### fr - Fuzzy Command History Search

**Rust 1.88.0** | **Linux/Bash only**

### Installation
1. Download binary:
    ```bash
    wget https://github.com/podvoyskiy/fr/releases/latest/download/fr -O ~/.local/bin/fr
    ```

2. Make executable:
   ```sh
   chmod +x ~/.local/bin/fr
   ```

3. Add to `~/.bashrc`:
    ```bash
    fr() { if [ $# -gt 0 ]; then ~/.local/bin/fr "$@"; else eval "$(~/.local/bin/fr)"; fi }
    ```

4. Reload bashrc:
    ```bash
    source ~/.bashrc
    ```

### Usage
```bash
fr  #Interactive search

fr --help  #show options
```

### Development Version

```bash
# Clone and setup:
git clone https://github.com/podvoyskiy/fr.git

cd fr

echo 'frdev() { if [ $# -gt 0 ]; then cargo run --quiet -- "$@"; else eval "$(cargo run --quiet --)"; fi }' >> ~/.bashrc

source ~/.bashrc
```

***Usage:*** `frdev` (same as `fr`)