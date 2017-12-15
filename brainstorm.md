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

### Thoughts:

- Perhaps, some sort of caching in any of the steps above?
- Can you learn the scheme instead of needing a X -> Y profile?
- You need to account for the defaults from various editors, such as defaults in vimscheme.

### TODO:

- Make the source file type requirement optional. Just infer from file extension. If file doesn't have it, then require it.
- Handle file IO/paths etc more idiomatically.
- Strings and company are handled atrociously right now. Please fix.
- Use write! macro for string building.
- Use ? wherever you can.
- Consequently, return Result<String, Error> and handle accordingly.
- Perhaps, use the reader.lines() method to read the file.
- Use traits: think about how to restructure the code for using with traits.
	- Instead of match, just pass in the object of whatever type and implement a convert trait for all objects.
- Rethink the need for the IR struct.
- Use Cow

### Wishlist:

- Handle paletton schemes that are not tetrads, gracefully.
- JSON for serialized IR, should also enable tweaking mapping between source and destination.
- Perhaps add an option to randomize the mappings.
- Add inspection of file name extension to figure out source format type. For now ask explicitly.
- Print a list of formats that can be passed to the commandline as parameters.
- Add reading schemes from urls, github, paletton and others.
- Handle scheme names properly.
- Examples, tests and samples.
- Another tool that gives you back schemes based on colors or moods that you can specify. Opportunity to put nltk to good use?
