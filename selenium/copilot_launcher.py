import logging
from selenium import webdriver
from selenium.webdriver.chrome.service import Service
from selenium.webdriver.common.by import By
from selenium.webdriver.common.keys import Keys
from selenium.webdriver.chrome.options import Options
import time
import pickle
import os

# Define base directory
base_dir = os.path.expanduser('~/copilot') 
data_dir  = os.path.join(base_dir, 'data') 
log_dir    = os.path.join(base_dir, 'log') 
log_path = os.path.join(log_dir, 'copilot.log')


os.makedirs(data_dir, exist_ok=True)
os.makedirs(log_dir, exist_ok=True)

# Configure logging 
logging.basicConfig(filename=log_path, level=logging.DEBUG, format='%(asctime)s %(levelname)s:%(message)s')
logging.info("Starting Copilot Script")

try:
    logging.info(f"Chrome options set: user-data-dir={data_dir}")
    
	# Setup Chrome options
    chrome_options = Options()
    chrome_options.add_argument(f"user-data-dir={data_dir}")
    chrome_options.add_argument("--kiosk")
    #chrome_options.add_argument("--no-sandbox")
    #chrome_options.add_argument("--disable-dev-shm-usage")

    #chrome_options.add_argument("homepage=about:blank")
    chrome_options.add_argument("--disable-extensions")
    chrome_options.add_experimental_option("excludeSwitches", ["enable-automation"])
    chrome_options.add_argument("--app=https://copilot.microsoft.com/onboarding")

	# Path to your chromedriver
    chrome_service = Service("/usr/bin/chromedriver")
	# Initialize the Chrome driver
    driver = webdriver.Chrome(service=chrome_service, options=chrome_options)
    logging.info("WebDriver initialized successfully")
except Exception as e: 
	logging.error("An error occurred", exc_info=True)



