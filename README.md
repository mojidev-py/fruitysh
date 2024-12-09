## fruitysh

fruitysh is a (probably) fast shell that has it's own scripting language, full featured piping, and extra features you will only find here.

To use this shell anywhere, move it to something like Users on windows or home on linux.

![Example of shell](https://github.com/mojidev-py/fruitysh/blob/main/{86E5F530-5921-4346-82A6-6773101D49AD}.png)

## How Do I make this my default shell?

### Linux
Add this executable's path (`$HOME/.cargo/bin/fruity.exe` is most likely) to the shells file, over at `etc/shells`.

Use `chsh -s fruity`, and your default shell will be changed to fruitysh.
