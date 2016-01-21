#abr2png

abr2png extracts sampled brushes from Adobe Photoshop's ABR files and saves them as PNGs.

It is derived from, and should handle exactly the same brushes as, GIMP 2.8, whose code for reading ABRs is in turn derived from abr2gbr. That means if GIMP doesn't load your brushes, this won't either :)

##Installation

abr2png requires Rust 1.6.

    $ git clone https://github.com/scurest/abr2png.git
    $ cd abr2png
    $ cargo build --release

##Usage

Basic usage is

    abr2png path/to/mybrushes.abr
    
which will create a directory called `mybrushes` in the current directory containing the extracted brush images (`0.png`, `1.png`, etc.). You may also specify the output location with `-o`

    abr2png path/to/mybrushes.abr -o my/new/brush/dir
    
abr2png will create the output directory; it should not exist beforehand. (This is just so that it won't clobber any of your files.)

The brush images will be 8-bit greyscale PNG files. Black represents transparency. This is the opposite convention of the one used by GIMP, so if you want to import the images as GIMP brushes you'll need to invert them first. (Using Imagemagick, you can do this in-place with `mogrify -negate my/new/brush/dir/*`.)

##See also

* **Marco Lamberto's [abr2gbr](http://the.sunnyspot.org/gimp/tools.html)**. Very similar, but doesn't handle ABR6 files, which was the impetus for making this.
* **GIMP 2.8**. Specifically [gimp-brush-load.c](https://github.com/GNOME/gimp/blob/2275d4b257e9de36f1ac749e591378e58b348754/app/core/gimpbrush-load.c), from which this program is derived.
* **Bill Scott's [abrMate](http://www.texturemate.com/abrMate)**. Windows-only freeware. Has many more features.
