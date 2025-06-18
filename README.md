# img

img is a simple image manipulation tool.
It aims to provide a cli tool allowing for quick image manipulation
as well as a rust library for applying simpla image filters.

# Features

img is a very young tool so support for different types of filters is limited to:

- blur (implemented using mean blur, more algorithms to come)
- grayscale filter

it also allows for simple image manipulation:

- crop
- resize (using nearest algorithm)

in terms of I/O, reading and writing is implemented for:

- png files:
  - reading is implemented for grayscale, grayscale with alpha, rgb and rgba images with pixel depth of eight.
  - writing always results in an rgba eight bit depth image

# Cli installation

clone the repository using git

```bash
git clone git@github.com:sobczal2/img.git

```

within repository directory invoke

```bash
cargo install --path crates/img-cli

```

to install the tool

# Cli Usage

## Print Help

```bash
img --help
```

## Blur:

```bash
img blur -i input.png -o output.png -r <radius> -a <algorithm> -s <sigma>
```

- radius - non-negative integer describing range of the
  blur kernel. Passing value of 0 results in 1x1 kernel, value 1 - 3x3, etc.
- algorithm - either "mean" or "gaussian"
- sigma - sigma value used in gaussian blur algorithm

## Grayscale

```bash
img grayscale -i input.png -o output.png
```

## Crop

```bash
img crop -i input.png -o output.png -s [width]x[height]+[offset_x]x[offset_y]
```

- width - width of the target image in pixels
- height - height of the target image in pixels
- offset_x - horizontal offset of the target image in pixels (counting from left edge of the original image)
- offset_y - vertical offset of the target image in pixels (counting from top edge of the original image)

### for example

```bash
img crop -i input.png -o output.png -s 100x200+10x20
```

results in an image 100x200 starting 10 pixels from the left edge and 20 pixels from the top edge

## Resize

```bash
img resize -i input.png -o output.png -s [width]x[height]
```

- width - width of the target image in pixels
- height - height of the target image in pixels

# Library usage

Image struct is the main struct holding image data. It holds RGBA images where each pixel value ranges from 0 to 255.

From there, you can follow code documentation.

# Parallelism

This project can be compiled with "parallel" feature flag which enables parallel processing in some places using rayon.
It is disabled by default since rayon's overhead usually makes filters run slower compared to non parallel version, but can be
enabled when necessary.
