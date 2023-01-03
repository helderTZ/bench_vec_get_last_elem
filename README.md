# Benchmark get Vec's last element

Benchmark two ways in Rust for getting a Vec's last element:
- via calling last() and then unwrapping
- via indexing len()-1

For one iteration, there is a substantial difference:
```
$ cargo run --release -- 10 1
Running benchmark with size 10 for 1 iterations.
Average duration of ' vec[vec.len()-1] '     : 125ns
Median duration of  ' vec[vec.len()-1] '     : 125ns
Mode duration of    ' vec[vec.len()-1] '     : 125ns
Average duration of ' *vec.last().unwrap() ' : 53ns
Median duration of  ' *vec.last().unwrap() ' : 53ns
Mode duration of    ' *vec.last().unwrap() ' : 53ns
```

For more than one iteration, cache effects kick in:
```
$ cargo run --release -- 10 1000
Running benchmark with size 10 for 1000 iterations.
Average duration of ' vec[vec.len()-1] '     : 28ns
Median duration of  ' vec[vec.len()-1] '     : 27ns
Mode duration of    ' vec[vec.len()-1] '     : 26ns
Average duration of ' *vec.last().unwrap() ' : 27ns
Median duration of  ' *vec.last().unwrap() ' : 26ns
Mode duration of    ' *vec.last().unwrap() ' : 25ns
```