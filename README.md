# New Project Initializer
Cli tool to init projects with custom file structure.

**Installing**
```
$ cargo install npi
```

```
USAGE:
    npi [OPTIONS] [SUBCOMMAND]

OPTIONS:
    -h, --help       Print help information
    -v, --verbose    Display every step
    -V, --version    Print version information

SUBCOMMANDS:
    help    Print this message or the help of the given subcommand(s)
    new     Create new directory with a name
```

**Examples**
```
$ cat ~/.npi/c.fsn
[Makefile]
output := {{name}}

default: build

build: clean
	gcc -Wall -g main.c -o ${output}

clean:
	rm -f ${output} 

run: build
	./${output}
[EOF]
[main.c]
#include <stdio.h>

int main(void){
    printf("Wait... This is not Rust...\n");
    return 0;
}
[EOF]
$ npi new hello_world c
Creating project "hello_world" of type "c"
$ cd hello_world/
$ make run
rm -f hello_world 
gcc -Wall -g main.c -o hello_world
./hello_world
Wait... This is not Rust...
```