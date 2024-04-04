# Hasher

Hasher is a command-line tool for computing hash values of files using various hashing algorithms.

## Usage

```bash
hasher <filename> [<options>]
```

### Options

- `-c, --check <HASH>`: Check if the computed hash matches the provided hash value.
- `-a, --algorithm <ALGORITHM>`: Compute hash using a specific algorithm (e.g., SHA-256, SHA-512).

## Examples

- **Compute all hashes of a file:**

```bash
hasher file.txt
```

- **Check if computed hash matches a given value:**

```bash
hasher file.txt -c <hash_value>
```

- **Compute hash using a specific algorithm:**

```bash
hasher file.txt -a sha512
```

## Installation

The tool is not available via a package manager now, but it will be available in the future

### Manual Installation

#### Prerequisites

- Rust (if not already installed, you can get it from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install))

#### Steps

1. **Clone the repository:**

```bash
git clone https://github.com/eliasnau/hasher.git
```

2. **Navigate to the project directory:**

```bash
cd hasher
```

3. **Build the project using Cargo:**

```bash
cargo build --release
```

4. After successful build, the executable will be available in the target/release directory.

##### For Linux

1. Copy the build into the bin directory

```bash
sudo cp target/release/hasher /usr/bin
```

2. Set Permissions:

```bash
sudo chmod +x /usr/bin/hasher
```

3. Verify Installation:

```bash
hasher --version
```

##### For MacOS

1. Copy the build into the bin directory

```bash
sudo cp target/release/hasher /usr/local/bin
```

2. Set Permissions:

```bash
sudo chmod +x /usr/local/bin/hasher
```

3. Verify Installation:

```bash
hasher --version
```

##### For Windows:

1. **Add the Executable to the System Path:** Windows uses the PATH environment variable to determine which directories to search for executable files when a command is entered in the command prompt. To make your program accessible from anywhere in the command prompt, you can add the directory containing the executable to the PATH.

- Open File Explorer and navigate to the directory containing your compiled executable (target\release\your_program.exe).
- Copy the path to this directory.
- Open the Start menu, type "environment variables", and select "Edit the system environment variables".
- In the System Properties window, click on the "Environment Variables" button.
- In the Environment Variables window, select the "Path" variable under "System variables" and click "Edit".
- Click "New" and paste the path to the directory containing your executable.
- Click "OK" to close the windows.

2. **Verify Installation:** Open a new command prompt window and type the name of your program (without the .exe extension). If everything is set up correctly, your program should execute as expected.
