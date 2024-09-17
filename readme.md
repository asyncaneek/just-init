A cli program to generate files and folders using jinja templates.

```
Usage: just-init.exe --source <SOURCE> --data <DATA> --output <OUTPUT>

Options:
  -s, --source <SOURCE>  Path to template file or folder
  -d, --data <DATA>      Path to json file containing jinja variables
  -o, --output <OUTPUT>  Path to output folder      
  -h, --help             Print help
  -V, --version          Print version
```

Details:

The internal program uses [Tera](https://crates.io/crates/tera) to generate files with jinja templates embedded within. A pre-processing step is done to generate the files from a path file that also may be described with jinja templates. 

For example:

If a template file at 

```cpp
// ../template/{{namespace}}/{{class}}.hpp

{{namespace}}::{{class}};
```

```json
// ../template/data.json

{
    "namespace":"just",
    "class":"Just"
}
```

then, the output of running the command
> `just-init -s ../template -d ../template/data.json -o ../out`
will be

```cpp
// ../out/just/Just.hpp

just::Just;
```