## fruitysh

fruitysh is a (probably) fast shell that has it's own scripting language, full featured piping, and extra features you will only find here.
(you will need to run this shell with the appropriate permissions on Windows, since Users/`your name` is a protected directory)

To use this shell anywhere, move it to something like Users on windows or home on linux.

![Example of shell](https://github.com/mojidev-py/fruitysh/blob/main/{86E5F530-5921-4346-82A6-6773101D49AD}.png)

## How Do I make this my default shell?

### Linux
Add this executable's path (`$HOME/.cargo/bin/fruity.exe` is most likely) to the shells file, over at `etc/shells`.

Use `chsh -s fruity`, and your default shell will be changed to fruitysh.

## Possible .fruityconf fields
The fruity shell allows you to customize a minimal amount of fields, and the possible ones are:
```
"AUTOCOMPLETIONS" = Whether the shell provides autocompletions for commonly used commands.,
"EXPERIMENTAL" = Whether to enable experimental features for the shell (Either a bool, or a string indicating what feature you want to enable),
"PROMPT_NAME_COLOR" = The hex representation of the color you want to change the fruitysh@(username) part of the prompt to,
"PROMPT_INPUT_COLOR" = The hex representation of the color you want to change the ">>" part of the prompt to.
```
