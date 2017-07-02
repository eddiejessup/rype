from glob import glob

import matplotlib.pyplot as plt
# from bokeh.plotting import figure, show, output_file
import numpy as np

def parse(f):
    line = f.readline()
    assert line == 'grid_dimens:\n', repr(line)
    line = f.readline()
    assert line == 'nx,ny\n', line
    grid_dimens = list(map(int, f.readline().split(',')))

    f.readline()

    line = f.readline()
    assert line == 'nr_glyphs:\n', line
    nr_glyphs = int(f.readline())

    f.readline()

    glyphs = {}
    for _ in range(nr_glyphs):
        line = f.readline()
        assert line == 'glyph_name:\n', line
        glyph_name = f.readline().strip()
        line = f.readline()
        assert line == 'glyph_dimens:\n', line
        line = f.readline()
        assert line == 'gx,gy\n', line
        glyph_dimens = list(map(int, f.readline().split(',')))
        line = f.readline()
        assert line == 'glyph_shape:\n', line
        glyph_shape_lst = [list(map(int, f.readline().split(','))) for _ in range(glyph_dimens[1])]
        glyph = np.array(glyph_shape_lst, dtype=np.bool)
        glyphs[glyph_name] = glyph
        f.readline()

    line = f.readline()
    assert line == 'nr_c_boxes:\n', line
    nr_c_boxes = int(f.readline())

    f.readline()

    line = f.readline()
    assert line == 'glyph_name,x,y\n', line

    c_boxes = []
    for _ in range(nr_c_boxes):
        parts = f.readline().split(',')
        glyph_name = parts[0].strip()
        origin = np.array(list(map(float, parts[1:])))
        c_box = (glyph_name, origin)
        c_boxes.append(c_box)
    return grid_dimens, glyphs, c_boxes

file_names = sorted(glob('dat/*.csv'))

grid_dimens, _, _ = parse(open(file_names[0]))
fig, ax = plt.subplots()
im = ax.imshow(np.zeros(grid_dimens, dtype=np.bool), vmin=0, vmax=1)

for file_name in file_names:
    print(file_name)
    f = open(file_name)
    grid_dimens, glyphs, c_boxes = parse(f)

    grid = np.zeros(grid_dimens, dtype=np.bool)

    for glyph_name, origin in c_boxes:
        glyph = glyphs[glyph_name]
        origin_i = (origin * grid_dimens).round().astype(np.int)
        termin_i = origin_i + glyph.shape
        grid[origin_i[0]:termin_i[0], origin_i[1]:termin_i[1]] += glyph

    im.set_data(grid.T)
    fig.savefig(file_name.replace('csv', 'png'))

# p = figure(x_range=(0, 1), y_range=(0, 1))
# p.image(image=[d], x=0, y=0, dw=1, dh=1, palette="Spectral11")

# show(p)
