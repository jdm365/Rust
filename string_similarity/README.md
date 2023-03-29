<h3>String Similarity Metrics</h3>

Simple library to calculate some niche string similarity metrics. Written in rust
for performance gains over pure python implementations.
**NOTE: Will convert all charachters to lowercase.

<h5>Example</h5>
```
from string_sim_metrics import *

x = 'antidisestablishmentarianism'
y = 'Hippopotomonstrosesquippedaliophobia'

wlev_dist = weighted_levenshtein_distance(
        x, 
        y, 
        insertion_cost=2, 
        deletion_cost=3,
        substitution_cost=1
        )
print(wlev_dist)

>> 39

```


Performance difference vs an identical implementation in pure python.
```
from string_sim_metrics import *
from time import perf_counter


x = 'antidisestablishmentarianism'
y = 'Hippopotomonstrosesquippedaliophobia'

N_ITERS = 100_000

start = perf_counter()
for idx in range(N_ITERS):
    sim_rust_wlev = weighted_levenshtein_distance(
            x, 
            y, 
            insertion_cost=1, 
            deletion_cost=2,
            substitution_cost=1
            )
end = perf_counter()
print(f'Rust wlev Elapsed time:            {end - start} seconds')
print(f'Rust wlev distance:                {sim_rust_wlev}\n')


start = perf_counter()
for idx in range(N_ITERS):
    sim_python_wlev = weighted_levenshtein_python(x, y, 1, 2, 1)
end = perf_counter()
print(f'Python wlev Elapsed time:          {end - start} seconds')
print(f'Python wlev distance:              {sim_python_wlev}\n')


>> Rust wlev Elapsed time:            0.2741983429878019 seconds
>> Rust wlev distance:                31

>> Python wlev Elapsed time:          14.970698102988536 seconds
>> Python wlev distance:              31

```
~55x performance increase.


Additional parameters for jarowinkler sim.
```
x = 'antidisestablishmentarianism'
y = 'Hippopotomonstrosesquippedaliophobia'

jw_sim = jaro_winkler_similarity(x, y, max_prefix_length=4, scaling_factor=0.1)
```



Standard Jaccard implementation.
**NOTE: Performance in python is equally good if you use instersection and union built
        in functions. They are already compiled.
```
x = 'antidisestablishmentarianism'
y = 'Hippopotomonstrosesquippedaliophobia'

jaccard_sim = jaccard_similarity(x, y)
```


Nothing else provided at the moment. Might consider adding more if needed/desired.
