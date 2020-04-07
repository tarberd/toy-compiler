# toy-compiler

## first-time users

- install LLVM
  - on Arch, run: `<your-package-manager> -S llvm`.

## commands

### help

enter the `toyc` folder, and:

```bash
cargo run -- -h
```

will show all possible commands

### emit LLVM intermediate representation

```bash
cargo run -- -e <file-name>
```
