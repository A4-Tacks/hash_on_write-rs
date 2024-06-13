A wrapper for storing hash results to avoid running costly hash functions
multiple times without modifying the value

Due to Rust's abstraction of the hash system,
it can currently only be implemented as a hash that uses another internal Hasher for values

### Example
```rust
use hash_on_write::How;
use std::collections::HashSet;

let mut x = How::new_default("foo".to_owned());

assert!(! How::is_hashed(&x));
HashSet::new().insert(&x);
assert!(How::is_hashed(&x));

How::make_mut(&mut x).push('!');
assert!(! How::is_hashed(&x));
assert_eq!(*x, "foo!");
```

## bench

```ignore
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 8.2s, or reduce sample count to 60.
no cache                time:   [81.408 ms 81.529 ms 81.673 ms]
                        change: [-0.7393% -0.4157% -0.1121%] (p = 0.01 < 0.05)
                        Change within noise threshold.
Found 5 outliers among 100 measurements (5.00%)
  2 (2.00%) low mild
  2 (2.00%) high mild
  1 (1.00%) high severe

Benchmarking cache key: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 5.3s, or reduce sample count to 90.
cache key               time:   [53.380 ms 53.608 ms 53.855 ms]
                        change: [-0.6099% +0.0048% +0.5883%] (p = 0.99 > 0.05)
                        No change in performance detected.
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe
```
