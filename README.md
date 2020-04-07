# toy-compiler

## first-time users

- install LLVM
  - on Arch, run: `<your-package-manager> -S llvm`.

## commands

### help

```bash
# will show all possible commands
cargo run -- -h
```

### emit LLVM intermediate representation

```bash
cargo run -- -e <file-name>
```

### link emitted LLVM intermediate representation

```bash
ld -static -o <out-file> -L`gcc -print-file-name=` /usr/lib/crt1.o /usr/lib/crti.o <in-file> /usr/lib/crtn.o --start-group -lc -lgcc -lgcc_eh --end-group
```

## examples

there are example files in the `examples` folder, check'em out :)
