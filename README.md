# uwu_rust
Uwu translator written in Rust.

The program is heavily expired by [uwu](https://git.sr.ht/~polanco/uwu).

# Program Logic
The program replaces letters with other letters.<br>
For example it replaces r & l with w, and na with nya. 

# Usage
Here is a basic example.
```bash
$ uwu_rust "The train is late."
De twain is wate.
```
The program finally supports files.
```bash
$ echo "The train is late." > test.txt
$ uwu_rust -f ./test.txt
De twain is wate.
```
The program does also support piping
```bash
$ echo "The train is late." | uwu_rust
De twain is wate.
```

# License
The program is licensed under [MIT](https://opensource.org/licenses/MIT).
