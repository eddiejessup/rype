# import matplotlib.pyplot as plt
from bokeh.plotting import figure, show, output_file
import numpy as np

d = np.loadtxt('out.csv', delimiter=',', dtype=np.uint8)
# plt.imshow(d)

p = figure(x_range=(0, 1), y_range=(0, 1))
p.image(image=[d], x=0, y=0, dw=1, dh=1, palette="Spectral11")

show(p)
