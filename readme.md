
# Text Occlusion

Hides text and binary data on text files using zero-width unicode characters.

This is an experiment for fun and it doesn't attempt to be a good solution, but rather to explore an idea.


## Caveats.

The data is visually hidden, but it's relatively simple to retrieve it, even if the method used to hide it is not known.

This is also a very ineficient process, because the resulting file is much larger than the input text file and the hidden data put together, due to the encoding overhead.

Finally, the effect is lost in any text editor that doesn't support Unicode.


## How to use.

Build the program, then run with the following options:

```
-t, --tailhide: Action Flag to hide the data file in the input data.
-r, --recover: Action Flag to retrieve hidden data.

-i, --input: Sets the input file. Required.
-o, --output: Sets the output file. If not set, will output to stdio.
-c, --hidefile: Sets the data file that will be hidden in the input. Required if hiding.

-h, --help: Prints the help.
```

## Contributing

Contributions are welcome.

If you have any suggestions, ideas, or improvements, feel free to create an issue or to submit a pull request.

## License

This project is licensed under the Apache License 2.0.

### Summary

- You are free to use, modify, and distribute this software.
- If you modify the software or create a derivative work, you must include the original copyright notice and a notice of the changes you made.
- The software is provided "as is," without warranties of any kind.
