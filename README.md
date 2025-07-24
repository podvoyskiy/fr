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

3. Create wrapper script `~/.local/bin/fr_wrapper`:
    ```bash
    #!/bin/bash
    [ $# -gt 0 ] && exec ~/.local/bin/fr "$@" || eval "$(~/.local/bin/fr)"
    ```

4. Set permissions:
    ```bash
    chmod +x ~/.local/bin/fr_wrapper
    ```

5. Add to `~/.bashrc`:
    ```bash
    alias fr='~/.local/bin/fr_wrapper'
    ```

### Usage
```bash
fr  #Interactive search
```