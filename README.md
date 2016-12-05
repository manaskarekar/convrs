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

#### About Paletton

- www.paletton.com is a really nice website that lets you come up with color palettes.
- For use with this converter, you would create a palette with "tetrad" colors, for now, and export as text.
- Copy paste that into a text file. Including the first blank line. I plan to strip out blank lines.
- I need to pick good colors from the generated palette or the output schemes. Another feature I want to add is allowing specification of color mapping between the source and destination scheme using json IR or something.

### Notes

- Right now all the code is very fragile, I need to go back and do it right once I've grokked rust.
- Also, in jedit-scheme files, something is messing up while cycling through various schemes. It is probably some fields
  I'm ignoring to include. Either that or some other scheme is messing up the settings for when example scheme loads.
- The colors used by the scheme need to be configured properly. Right now, I'm just assigning random colors to various components.

