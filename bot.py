import cv2
import os
import shutil
import pyautogui
import numpy as np
import pytesseract
import math

# read in data
sample = 'Screenshot.png'
read_image = cv2.imread(sample)
# cv2.imshow('Input Screenshot', read_image)

# preprocess image
grey_scale = cv2.cvtColor(read_image, cv2.COLOR_BGR2GRAY)
thresh1 = cv2.adaptiveThreshold(
    grey_scale, 255, cv2.ADAPTIVE_THRESH_GAUSSIAN_C, cv2.THRESH_BINARY_INV, 11, 2)
# cv2.imshow('Preprocessor', thresh1)

# find contours
contours1, _ = cv2.findContours(
    thresh1, cv2.RETR_TREE, cv2.CHAIN_APPROX_SIMPLE)
max_area = 0
c = 0
for i in contours1:
    area = cv2.contourArea(i)
    if area > 1000:
        if area > max_area:
            max_area = area
            best_cnt = i
            # read_image = cv2.drawContours(
            #     read_image, contours1, c, (0, 255, 0), 3)
    c += 1
# cv2.imshow('Found Contours', read_image)

# create a mask
mask = np.zeros((grey_scale.shape), np.uint8)
cv2.drawContours(mask, [best_cnt], 0, 255, -1)
cv2.drawContours(mask, [best_cnt], 0, 0, 2)
# cv2.imshow('Mask', mask)

# crop image to mask
out = np.zeros_like(grey_scale)
out[mask == 255] = grey_scale[mask == 255]
# cv2.imshow('New Image', out)

# prepare filesystem
if os.path.exists('out'):
    shutil.rmtree('out')
os.mkdir('out')
os.chdir('out')
# cv2.imwrite('puzzle.png', out)

# find bounding box
(x, y, w, h) = cv2.boundingRect(best_cnt)

dW = w // 9
dH = h // 9

print(f'Total bounding box: ({x},{y}), ({x+w},{y+w})')
print(f'Cell dimensions: {dW} x {dH}')

sudoku = np.zeros(81)

for i in range(0, 9):
    for j in range(0, 9):
        x1 = x + (dW * j)
        x2 = x1 + dW
        y1 = y + (dH * i)
        y2 = y1 + dH

        im_gray = out[y1:y2, x1:x2]

        # print(f'Bounding box for cell {i}-{j}: ({x1},{y1}), ({x2},{y2})')
        # cv2.imwrite(f'cell-{i}-{j}.png', im_gray)

        # process image
        _, cell_bin = cv2.threshold(
            im_gray, 130, 255, cv2.THRESH_BINARY + cv2.THRESH_OTSU)

        # Fill everything that is the same colour (black) as top-left corner with white
        cv2.floodFill(cell_bin, None, (0, 0), 255)
        cv2.floodFill(cell_bin, None, (1, 1), 255)

        # cv2.imwrite(f'cell-proc-{i}-{j}.png', cell_bin)

        if np.mean(cell_bin) > 250:
            text = '0'
        else:
            text = pytesseract.image_to_string(
                cell_bin, config='--psm 10 -c tessedit_char_whitelist=123456789').strip()

        # print(f'Cell {i}-{j}: {text}')

        sudoku[i * 9 + j] = int(text)

print(sudoku)

cv2.waitKey()
cv2.destroyAllWindows()
