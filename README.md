# Copilot

## Overview
**Copilot** is a wrapper application for Ubuntu that seamlessly launches the Copilot web interface. This tool aims to provide a similar experience to the native Copilot applications available for Windows, macOS, and Android.

## Features
- **Automatic Launch**: Open Copilot web interface with ease.
- **Simple Installation**: Easy setup and configuration through a script.
- **Lightweight**: Minimal resource usage and efficient performance.
- **Customizable**: Modify the script and settings to suit your needs.

## Installation
Follow these steps to install the Copilot wrapper:

1. **Clone the Repository**
    ```sh
    git clone https://github.com/israelsilvabarbara/copilot.git
    cd copilot
    ```

2. **Run the Setup Script**
    ```sh
    ./setup.sh install
    ```

3. **Launch Copilot**
    ```sh
    copilot
    ```

## Usage
- **Launch**: Use the `copilot` command to start the Copilot web interface.
- **Uninstall**: Run `./setup.sh uninstall` to remove the application.
- **Clean Environment**: Run `./setup.sh clean` to remove the virtual environment.

## Development
### Prerequisites
- Python 3
- Git

### Setting Up
1. **Clone the Repository**
    ```sh
    git clone https://github.com/israelsilvabarbara/copilot.git
    cd copilot
    ```

2. **Create a Virtual Environment and Activate It**
    ```sh
    python3 -m venv env
    source env/bin/activate
    ```

3. **Install Dependencies**
    ```sh
    pip install -r requirements.txt
    ```

### Contributing
Contributions are welcome! Please fork the repository and submit a pull request.

### License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contact
For any questions, please reach out to [israelsilvabarbara](mailto:israelsilvabarbara@gmail.com).
