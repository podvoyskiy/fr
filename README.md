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
    fr() { [ $# -gt 0 ] && ~/.local/bin/fr "$@" || eval "$(~/.local/bin/fr)"; }
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