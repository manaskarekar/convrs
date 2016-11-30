## Brainstorming.

### Possible Approach 1

- What you need:
	- A template for each (code editor + version).
	- An intermediate representation.
	- Scriptability.

### Notes

- Location of vim colors directory: /usr/share/vim/vim74/colors/
- Location of jedit schemes directory: /home/epic/.jedit/schemes
- Location of sublimetext schemes directory: ?


### Structure

- Vim scheme > Read > Use vim->IR profile > Convert to IR > Use IR->jedit profile > Write to file
- Structure of X -> Y profile and Y -> X profiles. Can you use a single file? One end will always be IR.
- Choosing an IR : JSON, yaml, toml, custom?
- Parse format1 into an IntermediateRepr object -> write to json ir file -> then read the file and spit it out as

### Thoughts:

- Perhaps, some sort of caching in any of the steps above?
- Can you learn the scheme instead of needing a X -> Y profile?
- You need to account for the defaults from various editors, such as defaults in vimscheme.

### TODO:

- Plugging in a simple json structure for now, hope to change it to a simple non json file.
- For v 0.0.1, pick out the stuff from a vim scheme that you need for jedit scheme.


- json_to_ir
- ir_to_json
- profiles
	- vim_reader and writer
	- jedit_reader and writer
- class IR
- class IRAttribute

- Strings and company are handled atrociously right now. Please fix.

### Wishlist:

- Add inspection of file name extension to figure out source format type. For now ask explicitly.
- Add a list of formats that can be passed to the commandline as parameters.
- Add reading schemes from urls, github, paletton and others.
- Handle scheme names properlu.