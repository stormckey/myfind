# myfind

My customized version of the `find` command implemented in Rust.

## Usage

```
./target/debug/myfind -p path1 -p path2 -r regex1 -r regex2 [-v|--asc|--desc|-h]
```

differenet regexes will be connected by `OR` operator.

use `-h` to get help.