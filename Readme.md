# abrupng

abrupng extracts image brushes from Adobe Photoshop's ABR files and saves them as PNGs.

## Installation

abrupng is written in [Rust](https://rust-lang.org/). Make sure you have Rust and Cargo installed.

### With `cargo install`

    $ cargo install --git https://github.com/scurest/abrupng.git

### By cloning the repo

    $ git clone https://github.com/scurest/abrupng.git
    $ cd abrupng
    $ cargo build --release

## Usage

Basic usage is

    abrupng path/to/mybrushes.abr
    
which will create a directory called `mybrushes` in the current directory containing the extracted brush images (`0.png`, `1.png`, etc.). You may also specify the output location with `-o`

    abupng path/to/mybrushes.abr -o my/new/brush/dir
    
abrupng will create the output directory; it should not exist beforehand. (This is just so that it won't clobber any of your files.)

The brush images will be 8-bit greyscale PNG files. Black represents transparency. This is the opposite convention of the one used by GIMP, so if you want to import the images as GIMP brushes you'll need to invert them first. (Using Imagemagick, you can do this in-place with `mogrify -negate my/new/brush/dir/*`.)

## What's with the dumb name?

abr + abrupt + png = abrupng?

...it was previously very logically called abr2png, but it turns out there was already [ZZYZX/abr2png](https://github.com/ZZYZX/abr2png), which fact incidentally would have been nice to find out sooner. (How did I not google that?)

## License

abrupng was derived from [gimp-brush-load.c](https://github.com/GNOME/gimp/blob/2275d4b257e9de36f1ac749e591378e58b348754/app/core/gimpbrush-load.c) in GIMP 2.8. It consequently is licensed under the GNU General Public License v3.0.
