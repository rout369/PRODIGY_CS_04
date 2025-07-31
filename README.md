CROSSKEY
CROSSKEY is a Rust-based keylogger tool designed for educational and ethical security testing purposes. It captures keystrokes and logs them to an encrypted file. A companion Python script, decrypter.py, is provided to decrypt the log file for analysis.

⚠️ Important: This tool is intended for ethical and legal use only, such as authorized security testing or educational purposes. Unauthorized use of keyloggers is illegal and unethical. Ensure you have explicit permission from the system owner before use.

Features

Captures and logs keystrokes securely.
Encrypts logs to protect sensitive data.
Includes a Python script to decrypt log files.
Cross-platform compatibility (Windows, Linux, macOS).

Screenshots
To be added in the future.

 Placeholder for setup screenshot
 Placeholder for tool usage screenshot
 Placeholder for decrypter script output screenshot

Prerequisites
Ensure the following are installed on your system:

Rust (version 1.65 or later, includes Cargo)
Git for cloning the repository
Python (version 3.8 or later, for the decrypter script)
[Optional: Any system-specific dependencies, e.g., libx11-dev on Ubuntu for X11-based keylogging]

Installation
Follow these steps to set up CROSSKEY locally:

Clone or Download the Repository
Clone the repository or download the zip file from GitHub:
git clone https://github.com/your-username/crosskey.git
cd crosskey

Alternatively, download and extract the zip file from [repository URL].

Verify Rust and Cargo Installation
Check if Rust and Cargo are installed:
rustc --version
cargo --version

If not installed, download Rust from rust-lang.org and follow the installation instructions.

Create a New Rust Project
Create a new Rust project named crosskey:
cargo new crosskey
cd crosskey


Replace main.rs Content
Navigate to crosskey/src/, delete the contents of main.rs, and copy the contents of PRODIGY_CS_04/src/keylogger.rs from the downloaded repository or zip file into main.rs.
cp ../PRODIGY_CS_04/src/keylogger.rs src/main.rs


Update Cargo.toml
Delete the contents of crosskey/Cargo.toml and replace them with the contents of PRODIGY_CS_04/Cargo.toml from the downloaded repository or zip file.
cp ../PRODIGY_CS_04/Cargo.toml .


Build the Project
Build the tool in release mode:
cargo build --release

This creates a binary in the target/release/ directory.

Run the Tool
Navigate to the release directory and run the tool with the help flag to see usage instructions:
cd target/release
./crosskey --help

On Windows, use:
crosskey.exe --help



Usage
Run the keylogger from the target/release/ directory:
./crosskey [OPTIONS]

Example
Start the keylogger and save logs to an encrypted file:
./crosskey --output keylog.enc

Run ./crosskey --help for a full list of options and commands.
Decrypting Logs
To read the encrypted log file, use the provided decrypter.py script:

Copy the Decrypter Script
Copy decrypter.py from the PRODIGY_CS_04/ directory to your working directory:
cp ../PRODIGY_CS_04/decrypter.py .


Run the Decrypter
Use Python to decrypt the log file (replace keylog.enc with your log file):
python3 decrypter.py keylog.enc

This outputs the decrypted keystrokes to the terminal or a specified file, depending on the script’s implementation.


Ethical Use

Use CROSSKEY only with explicit permission from the system owner.
This tool is for security researchers, penetration testers, or educational purposes.
Unauthorized keylogging violates privacy laws and ethical standards.

Troubleshooting

Rust/Cargo Not Found: Run rustup update to ensure Rust is up to date.
Build Errors: Verify that Cargo.toml dependencies are correct and system libraries are installed (e.g., sudo apt-get install libx11-dev on Ubuntu).
Decrypter Issues: Ensure Python 3.8+ is installed and check for required Python packages (e.g., pip install cryptography if used).

Contributing
Contributions are welcome! To contribute:

Fork the repository.
Create a feature branch (git checkout -b feature/your-feature).
Commit your changes (git commit -m "Add your feature").
Push to the branch (git push origin feature/your-feature).
Open a Pull Request.

License
This project is licensed under the MIT License. See the LICENSE file for details.
Contact
For issues or questions, open an issue on GitHub or contact [your contact info, e.g., your-username@example.com].
