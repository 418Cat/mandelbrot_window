# Mandelbrot fractal viewer

It's a small program made in rust to view the mandelbrot fractal and output images.


## Keybinds :
- lmb click     : center view on click
- scroll        : zoom out and in
- up arrow      : zoom in
- down arrow    : zoom out
- left arrow    : lower the max iteration limit
- right arrow   : raise the max iteration limit
- Return        : output an image to the output directory
- numpad enter  : output an image to the output directory
- space         : output a series of images to the output, zooming out image by image until the width is more than 2


## Config
- res:          : [u32; 2]  : [x, y] resolution of the window
- img_output_res: [u32; 2]  : [x, y] resolution of the output image
- start_rect    : [f64; 2]  : [x, y] coordinates of the top left corner
- rect_size     : [f64; 2]  : [x, y] size of the displayed image
- limit         : u32       : max iteration limit
- zoom          : f64       : zoom divider, the higher the value, the slower the zoom
- color_fn      : enum      : color function to use to display the mandelbrot fractal. Currently from 'Colors1' to 'Colors6'
