#!/bin/bash

# Exit immediately if a command exits with a non-zero status 
set -e

# Variables
APP_NAME="copilot"
DATA_DIR="$HOME/$APP_NAME"
SELENIUM_DIR="$DATA_DIR/data"
ENV_DIR="$DATA_DIR/env"
LOG_DIR="$DATA_DIR/log"

BIN_NAME="copilot_launcher"
SCRIPT_NAME="copilot_launcher.py"
ICON_NAME="$APP_NAME.svg"
DESKTOP_NAME="$APP_NAME.desktop"

PATH_TO_BIN="/usr/local/bin"
PATH_TO_DESKTOP="/usr/share/applications"
PATH_TO_ICONS="/usr/share/icons/$APP_NAME"

function show_help {
    echo "Usage: $0 {install|uninstall|clean|--help}"
    echo "   install       Install the application"
    echo "   uninstall     Uninstall the application"
    echo "   clean         Remove the virtual environment"
    echo "   chrome-setup  Check and install ChromeDriver if necessary"
    echo "   --help        Display this help message"
}

function check_chromedriver { 
    if ! which chromedriver &> /dev/null; then
        echo "#chromedriver could not be found. Installing it..."
        sudo apt-get install chromium-chromedriver
    else
        echo "#chromedriver is already installed."
    fi
}

function check_for_python {
    if dpkg -l | grep -q "python3.*"; then
        echo "Python 3 packages are already installed."
    else
        echo "Python 3 packages are not installed. Installing Python 3 and python3-venv..."
        sudo apt-get install -y python3 python3-venv
    fi

    if dpkg -l | grep -q "python.*-venv"; then
        echo "python-venv package is already installed."
    else
        echo "python-venv package is not installed. Installing it..."
        sudo apt-get install -y python3-venv
    fi
}



function check_environment {
    # Check if the virtual environment directory and bin/activate file exist
    if [ ! -f "$ENV_DIR/bin/activate" ]; then
        echo "Creating virtual environment..."
        python3 -m venv $ENV_DIR
        echo "Virtual environment created!"
    else
        echo "Virtual environment already exists."
    fi

    echo "Activating virtual environment..."
    source $ENV_DIR/bin/activate

    # Check if required packages are installed
    if ! pip show pyinstaller &> /dev/null || ! pip show selenium &> /dev/null; then
        echo "Required packages are not fully installed. Installing them..."
        pip install pyinstaller selenium
        echo "Required packages are installed!"
    else
        echo "Required packages are already installed."
    fi
}


function install_app {
    echo "Installing the application..."
    sudo apt-get update

    echo "Checking for drivers..."
    # Check if chromedriver is installed
    check_chromedriver

    echo "Creating directories with sudo..."
    # Ensure directories have the correct permissions
    sudo mkdir -p $DATA_DIR
    sudo mkdir -p $SELENIUM_DIR
    sudo mkdir -p $ENV_DIR
    sudo mkdir -p $LOG_DIR
    sudo mkdir -p $PATH_TO_ICONS
    
    sudo chmod -R u+w $DATA_DIR
    sudo chown -R $USER:$USER $DATA_DIR

    echo "Checking for Python"
    check_for_python

    check_environment

    echo "Creating venv..."  
    # Step 1: Create the environment for Python
    sudo python3 -m venv $ENV_DIR

    # Step 2: Activate the environment and install PyInstaller
    echo "Installing pyinstaller..."
    source $ENV_DIR/bin/activate
    sudo $ENV_DIR/bin/pip install pyinstaller selenium

    # Get the full path to the pyinstaller executable 
    PYINSTALLER_PATH=$(which pyinstaller)
    echo "Building the executable from $SCRIPT_NAME..." 
   #$PYINSTALLER_PATH --onefile $SCRIPT_NAME
    $PYINSTALLER_PATH --onefile --icon=copilot.png $SCRIPT_NAME
    # Step 4: Copy the .desktop file, executable, and icon to the appropriate directories
    echo "Copying files into destination..."
    sudo cp dist/$BIN_NAME $PATH_TO_BIN/$APP_NAME
    sudo cp $ICON_NAME $PATH_TO_ICONS/$ICON_NAME
    sudo cp $DESKTOP_NAME $PATH_TO_DESKTOP/$DESKTOP_NAME
    
    # Make sure the desktop entry file is executable
    sudo chmod +x $PATH_TO_DESKTOP/$DESKTOP_FILE

    echo "Installation complete!"
}

function uninstall_app {
    echo "Uninstalling the application..."
    
    set +e
    # Remove the executable 
    sudo rm $PATH_TO_BIN/$APP_NAME 
    # Remove the icon 
    sudo rm $PATH_TO_ICONS/$ICON_NAME 
    # Remove the .desktop file 
    sudo rm $PATH_TO_DESKTOP/$DESKTOP_NAME
    
    # Remove the data folder
    sudo rm -r $DATA_DIR
    set -e
    
    echo "Uninstallation complete!"
}

function clean_env {
    echo "Removing the virtual environment..."
    sudo rm -rf $ENV_DIR
    echo "Environment removed!"
}

case "$1" in
    install)
        install_app
        ;;
    uninstall)
        uninstall_app
        ;;
    clean)
        clean_env
        ;;
    chrome-setup)
        check_chromedriver
        ;;
    --help)
        show_help
        ;;
    *)
        echo "Invalid option: $1"
        show_help
        ;;
esac

