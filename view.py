from glob import glob

import matplotlib.pyplot as plt
# from bokeh.plotting import figure, show, output_file
import numpy as np

fig, ax = plt.subplots()

def parse(f):
    line = f.readline()
    assert line == 'grid_dimens:\n', repr(line)
    line = f.readline()
    assert line == 'nx,ny\n', line
    grid_dimens = list(map(int, f.readline().split(',')))

    f.readline()

    line = f.readline()
    assert line == 'glyph_dimens:\n', line
    line = f.readline()
    assert line == 'gx,gy\n', line
    glyph_dimens = list(map(int, f.readline().split(',')))

    f.readline()

    line = f.readline()
    assert line == 'glyph_shape:\n', line
    glyph_shape_lst = [list(map(int, f.readline().split(','))) for _ in range(glyph_dimens[1])]
    glyph = np.array(glyph_shape_lst, dtype=np.bool)

    f.readline()

    line = f.readline()
    assert line == 'nr_glyph_instances:\n', line
    nr_glyphs = int(f.readline())

    f.readline()

    line = f.readline()
    assert line == 'glyph_instance_origins:\n', line
    line = f.readline()
    assert line == 'i,j\n', line
    glyph_origins_lst = [list(map(int, f.readline().split(','))) for _ in range(nr_glyphs)]
    glyph_origins = np.array(glyph_origins_lst, dtype=np.int)
    return grid_dimens, glyph, glyph_origins

file_names = sorted(glob('dat/*.csv'))
for file_name in file_names:
    print(file_name)
    f = open(file_name)
    grid_dimens, glyph, glyph_origins = parse(f)
    
    grid = np.zeros(grid_dimens, dtype=np.bool)
    for glyph_origin in glyph_origins:
        glyph_termin = glyph_origin + glyph.shape
        grid[glyph_origin[0]:glyph_termin[0], glyph_origin[1]:glyph_termin[1]] += glyph

    ax.cla()
    ax.imshow(grid.T)
    fig.savefig(file_name.replace('csv', 'png'))

# p = figure(x_range=(0, 1), y_range=(0, 1))
# p.image(image=[d], x=0, y=0, dw=1, dh=1, palette="Spectral11")

# show(p)
