# burau4_mod_p

A simple program for finding kernel elements of the 4-strand Burau representation modulo $p$.

## Example usage

To build this, run
```
$ cargo build --release
```

Then, to find an element of Burau mod 2:
```
$ time target/release/burau4_mod_p 2
Found kernel element. Garside generators:
[13, 10, 13, 10, 13, 10, 13, 10]

real	0m0.010s
user	0m0.010s
sys	0m0.000s
```
Or Burau mod 3:

```
$ time target/release/burau4_mod_p 3
Found kernel element. Garside generators:
[7, 8, 16, 13, 20, 13, 10, 13, 4, 13, 20, 17, 22, 17, 13, 10, 13, 8, 13, 11, 13, 10, 16, 3]

real	0m4.404s
user	0m4.192s
sys	0m0.212s
```
