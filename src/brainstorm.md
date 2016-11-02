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

### Misc notes:

- Perhaps, some sort of caching in any of the steps above?
- Can you learn the scheme instead of needing a X -> Y profile?

### TODO:

- Plugging in a simple json structure for now, hope to change it to a simple non json file.