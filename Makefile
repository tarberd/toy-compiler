INPUT_FILE?=examples/argument.toy
OUTPUT_FILE?=executable

compile:
	cargo run -- -e $(INPUT_FILE)
	ld -static -o bin/$(OUTPUT_FILE) -L`gcc -print-file-name=` /usr/lib/crt1.o /usr/lib/crti.o abc /usr/lib/crtn.o --start-group -lc -lgcc -lgcc_eh --end-group
