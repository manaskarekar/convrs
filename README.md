# Convrs

*A tool to convert color schemes between editors.*

...

This is a 'get my hands dirty with Rust' toy project, far from idiomatic, or efficient Rust, but fun. In places, I know how I would do it differently in other languages, but haven't gone deep down that rabbit hole in Rust. Please feel free to point out what I'm doing inefficiently/incorrectly, it'll help me learn quicker!


### Features

- Hopes to enable converting color schemes between different editors and palettes.
- In the future, hope to allow better mapping from source colors to destination colors.
- Right now, support for text files spit out by http://paletton.com/ to be spit out as jEdit scheme files.

### General use

- ./convrs < source_file > < source_format > < destination_format >

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