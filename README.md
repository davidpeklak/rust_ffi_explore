# Usage
- Clone the project.
- Create two named pipes named pipe1 and pipe2:
```shell
mkfifo pipe1
mkfifo pipe2
```
- Run the project
```shell
cargo run
```
- In a second terminal, write into the named pipes, e.g.
```shell
echo foobar > pipe1
echo foobar > pipe2
```
- Keep writing into the named pipes. When aou have written >= 100 characters into one of the pipes, observe a message on the console of the running process:
```shell
Future1 finished with...
```
or
```shell
Future2 finished with...
```
# Literature
- [Futures Explained in 200 Lines of Rust](https://cfsamson.github.io/books-futures-explained)