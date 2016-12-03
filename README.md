### Convrs 

*A tool to convert color schemes between editors.*

...

This is a hara-kiri toy project to get my hands dirty with Rust, far from idiomatic rust, but fun. 


# Features

- Hopes to enable converting color schemes between different editors and palettes.
- Right now, support for text files spit out by http://paletton.com/ to be spit out as jEdit scheme files. The colors used by the scheme need to be configured properly. Right now, just assigning random colors to various components.
- 

# General use

- ./convrs <source_file> source_format destination_format

# Example

- ./convrs paletton_example1.txt paletton_txt jedit

# Notes

- Right now all the code is very fragile, I need to go back and do it right once I've grokked rust.

### Paletton

- This is a really nice website that lets you come up with color palettes.
- For use with this converter, you would create a palette with "tetrad" colors, for now, and export as text.
- Copy paste that into a text file. Including the first blank line. I plan to strip out blank lines.
- I need to pick good colors from the generated palette or the output schemes. Another feature I want to add is allowing specification of color mapping between the source and destination scheme using json IR or something.

