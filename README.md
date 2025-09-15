# img

img is a simple image manipulation tool.
It aims to provide a cli tool allowing for quick image manipulation
as well as a rust library for applying image filters.

# Features

img is a very young tool so support for different types of filters is limited to:

- blur (available algorithms: mean blur, gaussian blur)
- grayscale filter
- sepia filter
- gamma correction filter
- canny edge detection
- kuwahara filter

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

to install the tool.

By default `parallel` feature is enabled which allows each operation to specify `-t <thread_count>` parameter
where thread_count is either a positive number or `auto` (which is the default and uses number equal to logical
cores available).

# Cli Usage

## Print Help

```bash
img --help
```

## Blur:

```bash
img blur -i input.png -o output.png [algorithm] -r <radius> -s <sigma>
```

- algorithm - either "mean" or "gaussian"
- radius - non-negative integer describing range of the
  blur kernel. Passing value of 0 results in 1x1 kernel, value 1 - 3x3, etc.
- sigma - sigma value used in gaussian blur algorithm

## Grayscale

```bash
img grayscale -i input.png -o output.png
```

## Sepia

```bash
img sepia -i input.png -o output.png
```

## Gamma Correction

```bash
img gamma-correction -i input.png -o output.png -g <gamma>
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

## Canny

```bash
img canny -i input.png -o output.png
```

## Canny

```bash
img canny -i input.png -o output.png
```

## Kuwahara

```bash
img kuwahara -i input.png -o output.png
```

# Library usage

Image struct is the main struct holding image data. It holds RGBA images where each pixel value ranges from 0 to 255.

All filters use lens api (see [Lens trait](https://github.com/sobczal2/img/blob/189db3ba2c98e30223362a5ffcdfda4ab53fb9e3/crates/img/src/lens/mod.rs#L52).
This is a main way of interacting with image, each `Lens` transforms each point of an `Image` (or a different 2d
representation). This api is lazy, inspired by `Iterator` so it does not perform any expensive calculations
unless [`Lens::look`](https://github.com/sobczal2/img/blob/189db3ba2c98e30223362a5ffcdfda4ab53fb9e3/crates/img/src/lens/mod.rs#L90) method is called.

From there, you can follow code documentation.

# Parallelism

This project can be compiled with "parallel" feature flag which adds corresponding functions utilizing parallelism, most important being
[`FromLensPar::from_lens_par` method](https://github.com/sobczal2/img/blob/189db3ba2c98e30223362a5ffcdfda4ab53fb9e3/crates/img/src/lens/mod.rs#L307).
