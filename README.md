# Convrs

*A tool to convert color schemes between editors.*

### Features

- Convert color schemes between different formats.
- In the future, hope to allow better mapping from source colors to destination colors.
- Right now, support for text files spit out by http://paletton.com/ to be spit out as jEdit scheme files.

### General use

- ./convrs < source_file or source_folder > < source_format > < destination_format >

### Example

- ./convrs paletton_example1.txt paletton_txt jedit

### Supported Conversions

#### Input

- paletton.com - Generated "tetrad" palettes.
- TODO: add vim, SublimeText and Textmate themes.

#### Output

- jEdit schemes.

### Notes

- The colors used by the scheme need to be configured properly. Right now, I'm just assigning random colors to various components.
- I need to pick good colors from the generated palette or the output schemes.
- www.paletton.com is a really nice website that lets you come up with color palettes.
- If you want to use paletton with this converter, you would create a palette with "tetrad" colors, for now, and export as text. Copy paste
  that into a text file. Include the first blank line in the text file.
