# Sudoku Bot

This was a project created by Nathan Rowan to experiment with OpenCV, `pyautogui` and implementing Knuth's Algorithm X in Rust. The bot is able to complete Sudoku.com puzzles on "evil" difficulty in 15 seconds.

# How it works

The main logic is written in [bot.py](bot.py). The general steps are:

 1. Switch to browser and start new puzzle
 2. Take a screenshot
 3. Use OpenCV to find the bounds of the puzzle within the screenshot
 4. Use Tesseract OCR on each cell of the puzzle and store into a list
 5. Use Knuth's Algorithm X with Dancing Links to solve the puzzle (implemented in Rust)
 6. Click on each empty cell and enter the number found in the solution

# Building

This project is built using Maturin. Make sure to have Rust, Maturin and Python installed. Steps to build:

 1. Create a new Python venv:
    
    ```bash
    python3 -m venv .venv
    source .env/bin/activate
    ```

 2. Build the Rust library:

    ```bash
    maturin build
    ```

 3. Install the Python wheel:
   
    ```bash
    maturin develop
    ```

 4. Install dependencies:

   ```bash
   pip install pyautogui numpy pytesseract opencv-python
   ```

 5. Run the bot:

    ```bash
    python3 bot.py
    ```
