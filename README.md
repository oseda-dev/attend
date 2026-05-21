# Attend

Attend is a lightweight Rust-based classroom attendance tracking system. 
It generates a local web server and a corresponding QR code to allow users note their attendance via a web interface.
Attendance logs are stored *locally* as CSV files organized by class and date.
No student information ever leaves your system.

---

### Prerequisites

* **Rust Toolchain:** You will need cargo installed to build the binary.
* Will publish to cargo soon.

---

### Configuration

The application looks for a configuration file named **attend.conf** located in your `Attend` directory. By default, this directory is:

- The path specified in the **ATTEND_HOME** environment variable.
    - This will default your home directory if not set


### bash
```bash
echo 'export ATTEND_HOME="~/Attend"' >> ~/.bashrc
```

### zsh
```bash
echo 'export ATTEND_HOME="~/Attend"' >> ~/.zshrc
```

### fish
```fish
set -Ux ATTEND_HOME ~/Attend
```

### powershell
```powershell
Add-Content -Path $PROFILE -Value '$env:ATTEND_HOME = "$HOME\Attend"'
```

#### Setup attend.conf
Create the file and list your classes, one per line:
* Mathematics
* Computer Science 101
* Physics

---

### Usage

1. **Launch the application:** Run the binary from your terminal.
2. **Select Class:** from the items defined in your `attend.conf`
3. **Select Date:** Choose the date for the session. This ultimately defines where to log the data to. (defaults to current system date)
4. **Display the QR Code:**. A QR will be opened in your default application. It also gets saved to the path where the log file is present. 
5. **Record Attendance:** Students can scan the code to access the web interface and enter their emails. A session ID is stored in their browser to detect duplicate submissions.

1. Stats can also be checked via `attend check`
1. You may have to enable your port on the firewall (see `firewall-update.sh`)
---


### Data Storage

Attendance data is saved in a structured format within your Attend home directory:
`[HOME]/[CLASS]/[DATE]/log.csv`

---
