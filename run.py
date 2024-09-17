import sys
import polars as pl
from io_plugin import new_bernoulli, new_uniform, new_fibonacci, new_reader, scan_random, scan_sequence, scan_files


lf = scan_random(
    [
        new_bernoulli("b0.5", p=0.5, seed=1),
        new_bernoulli("b0.1", p=0.1, seed=2),
        new_uniform("uniform", low=10, high=100, dtype=pl.Int32, seed=2),
    ]
)

#print(lf.collect())

# predicate pushdown
assert lf.filter(pl.col("b0.5")).collect()["b0.5"].all()

# slice pushdown
assert lf.head(100).collect().shape == (100, 3)

# projection pushdown
out = lf.select("uniform", "b0.1").collect()
assert out.shape == (1000, 2)
assert out.columns == ["uniform", "b0.1"]

lf = scan_sequence(
    [
        new_fibonacci("fib_pos", first=1, second=1),
        new_fibonacci("fib_neg", first=-1, second=-1),
    ]
)

## Don't
#lf = lf.filter(pl.col("fib_pos") > 10)

#print()
#print("--- Fibonacci sequence plugin ---")
#print("lf.head(10).collect():")
#print(lf.head(10).collect())
#print()
#print("lf.head(10).collect():")
#print(lf.head(10).collect())


lf = scan_files(
    [
        new_reader("first", sys.argv[1] if len(sys.argv) > 1 else "/dev/null"),
    ]
)

with pl.Config(
    fmt_str_lengths=1000,
    tbl_width_chars=1000,
    tbl_rows=1000,
):
    print()
    print("--- Line reader plugin ---")
    print("lf.head(3).collect():")
    print(lf.head().collect())
