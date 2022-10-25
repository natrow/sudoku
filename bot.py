# Copyright 2022 Nathan Rowan
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#   http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing,
# software distributed under the License is distributed on an
# "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND,
# either express or implied. See the License for the specific
# language governing permissions and limitations under the License.

import pyautogui as gui
import numpy as np
import pytesseract as tess
import cv2
import os
import sudoku
import time

start_time = time.time()

# hardcoded offsets for board position on screen
offset_x = 300
offset_y = 342

width = 550
height = 550

# screenshot name
screenshot_name = 'Screenshot.png'

# clear old screenshots
if os.path.exists(screenshot_name):
    os.remove(screenshot_name)

# switch windows
gui.hotkey('win', '1')

# start recording
# gui.hotkey('win', 'r')

# start new game
gui.click(1024, 379, duration=0.1)
# gui.click(930, 649, duration=0.1) # easy
# gui.click(930, 700, duration=0.1) # medium
# gui.click(930, 766, duration=0.1) # hard
# gui.click(930, 815, duration=0.1) # expert
gui.click(930, 875, duration=0.1)  # evil
time.sleep(1)

# take a screenshot
gui.screenshot('Screenshot.png', region=(offset_x, offset_y, width, height))

# load screenshot into OpenCV
img = cv2.imread(screenshot_name)

print(f'Time to read screenshot: {time.time() - start_time}')
start_time = time.time()

# preprocess image
img = cv2.cvtColor(img, cv2.COLOR_BGR2GRAY)
thresh = cv2.adaptiveThreshold(
    img, 255, cv2.ADAPTIVE_THRESH_GAUSSIAN_C, cv2.THRESH_BINARY_INV, 11, 2)

# find puzzle bounds
contours, _ = cv2.findContours(
    thresh, cv2.RETR_TREE, cv2.CHAIN_APPROX_SIMPLE)
contour_areas = list(map(cv2.contourArea, contours))
largest_contour = contours[contour_areas.index(max(contour_areas))]
(x, y, w, h) = cv2.boundingRect(largest_contour)
dW = w // 9
dH = h // 9

print(f'Time to process image: {time.time() - start_time}')
start_time = time.time()

# use OCR to fill in puzzle
unsolved_puzzle = np.zeros(81).astype(int)
for i in range(0, 9):
    for j in range(0, 9):
        # calculate cell bounds (3px border trimmed)
        x1 = x + (dW * j) + 3
        x2 = x1 + dW - 6
        y1 = y + (dH * i) + 3
        y2 = y1 + dH - 6
        cell = thresh[y1:y2, x1:x2]

        # if cell isn't empty use OCR
        if np.max(cell) < 200:
            text = '0'
        else:
            text = tess.image_to_string(
                cell, config='--psm 10 -c tessedit_char_whitelist=123456789').strip()

        # save cell to puzzle
        unsolved_puzzle[i * 9 + j] = int(text)

print(f'Time to extract OCR: {time.time() - start_time}')
start_time = time.time()

# solve sudoku puzzle
solved_puzzle = sudoku.solve(unsolved_puzzle)

print(f'Time to solve puzzle: {time.time() - start_time}')
start_time = time.time()

# fill in puzzle
for i in range(0, 9):
    for j in range(0, 9):
        # calculate center of cell
        x1 = x + dW * (j + 0.5) + offset_x
        y1 = y + dH * (i + 0.5) + offset_y

        if unsolved_puzzle[i * 9 + j] == 0:
            # click on cell
            gui.click(x1, y1)
            # enter key
            gui.press(str(solved_puzzle[i * 9 + j]))

print(f'Time to input answers: {time.time() - start_time}')

# stop recording
# time.sleep(1.0)
# gui.hotkey('win', 'r')
