# Keep Init 🛠️

**Keepinit** is a CLI tool designed to log your installation and command-line processes into organized session-based log files. It helps you keep track of your terminal activities for better documentation and troubleshooting.

## Features
- Start a logging session with a custom session name.
- Logs all executed commands and their outputs.
- Automatically skips logging commands related to passwords for security.
- Outputs logs to a file named `<session_name>.log`.

## Installation
1. Clone the repository:
   ```bash
   git clone https://github.com/Y3454R/keepinit.git
   cd keepinit
   ```
2. Build the project using Cargo:
   ```bash
   cargo build --release
   ```
3. Add the compiled binary to your system path for easier use:
   ```bash
   sudo cp target/release/keepinit /usr/local/bin
   ```

## Usage
Start a logging session:
```bash
keepinit start [session_name]
```
- `session_name` (optional): Specify a name for your session. Default is `session`.

Exit the session:
```bash
exit
```

Logs will be saved in `<session_name>.log` in the current working directory.

### Example
```bash
keepinit start my_session
> sudo apt update
> sudo apt install -y cowsay
> cowsay "Keepinit is awesome!"
> sudo apt remove -y cowsay
> exit
```

Log file `my_session.log`:
```
Session started: 2024-12-24 12:00:00

> sudo apt update
... (command output) ...

> sudo apt install -y cowsay
... (command output) ...

> cowsay "Keepinit is awesome!"
Keepinit is awesome!

> sudo apt remove -y cowsay
... (command output) ...

Session ended: 2024-12-24 12:05:00
```

## Future Updates
- Support for resuming an existing session.
- Logging commands from nested environments (e.g., Docker containers).
- Enhanced log format with timestamps for each command and output.
- Real-time filtering or searching within session logs.
- Compatibility with non-`sh` shells like `bash`, `zsh`, or `fish`.

## Contributions
Contributions, issues, and feature requests are welcome! Feel free to fork the repository and submit pull requests.

## License
[MIT License](LICENSE)
