Run benchmark with `cargo bench`

```
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

Benchmarking get inner 1
Benchmarking get inner 1: Warming up for 3.0000 s
Benchmarking get inner 1: Collecting 100 samples in estimated 5.0000 s (8.1B iterations)
Benchmarking get inner 1: Analyzing
get inner 1             time:   [605.89 ps 613.37 ps 622.46 ps]
                        change: [-3.9308% -2.9750% -2.0050%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 12 outliers among 100 measurements (12.00%)
  9 (9.00%) high mild
  3 (3.00%) high severe

Benchmarking get inner 2
Benchmarking get inner 2: Warming up for 3.0000 s
Benchmarking get inner 2: Collecting 100 samples in estimated 5.0000 s (8.1B iterations)
Benchmarking get inner 2: Analyzing
get inner 2             time:   [611.31 ps 612.11 ps 612.96 ps]
                        change: [-6.2371% -5.4176% -4.6859%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 9 outliers among 100 measurements (9.00%)
  2 (2.00%) low severe
  3 (3.00%) low mild
  1 (1.00%) high mild
  3 (3.00%) high severe

Benchmarking get inner 3
Benchmarking get inner 3: Warming up for 3.0000 s
Benchmarking get inner 3: Collecting 100 samples in estimated 5.0000 s (814M iterations)
Benchmarking get inner 3: Analyzing
get inner 3             time:   [6.1738 ns 6.2423 ns 6.3421 ns]
                        change: [-2.1447% -0.5987% +0.8941%] (p = 0.47 > 0.05)
                        No change in performance detected.
Found 13 outliers among 100 measurements (13.00%)
  2 (2.00%) low mild
  6 (6.00%) high mild
  5 (5.00%) high severe

Benchmarking get inner 4
Benchmarking get inner 4: Warming up for 3.0000 s
Benchmarking get inner 4: Collecting 100 samples in estimated 5.0000 s (807M iterations)
Benchmarking get inner 4: Analyzing
get inner 4             time:   [6.1370 ns 6.1608 ns 6.1997 ns]
                        change: [-5.8067% -4.6507% -3.6858%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) low severe
  1 (1.00%) low mild
  1 (1.00%) high mild
  3 (3.00%) high severe

Benchmarking get inner 5
Benchmarking get inner 5: Warming up for 3.0000 s
Benchmarking get inner 5: Collecting 100 samples in estimated 5.0000 s (5.6B iterations)
Benchmarking get inner 5: Analyzing
get inner 5             time:   [888.47 ps 890.83 ps 893.51 ps]
                        change: [-3.0874% -1.5699% -0.1424%] (p = 0.04 < 0.05)
                        Change within noise threshold.
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) low mild
  4 (4.00%) high mild
  2 (2.00%) high severe
```
