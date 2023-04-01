import numpy as np
from string_sim_metrics import *
from math import floor
from time import perf_counter
import pandas as pd


N_ITERS = 100000

x = 'abcdefghijklmnopqrstuvw'
y = 'zyxwvuasdflkjasdhgfedcb'

#x = 'abcdefasdlk;fas;dlkjfkjl;sadfkjl;sadfklj;ka;jlfsdghijklmnopqrstuvwxyzaklsjdflaksjjdfkjlasdklfj'
#y = 'zyxda;slkfdakjl;k;ljdfkjl;asdlk;jdfsakl;jadfskl;jfdsalk;jwvuasdflkjasdhgfedcbaaskjldfalskdfkjlasdkfljaasdklf;lkjasdkl;jfklj;asdkflj;asklj;dk;ljfakl;jslk'

## Function to calculate the weighted levenshtein similarity
def weighted_levenshtein_python(
        str1, 
        str2, 
        insertion_cost=1, 
        deletion_cost=1, 
        substitution_cost=1
        ):
    if str1 is None or str2 is None:
        return 0.0

    if str1 == str2:
        return 1.0

    len1 = len(str1)
    len2 = len(str2)

    if len1 == 0 or len2 == 0:
        return 0.0

    table = [[0] * (len2 + 1) for _ in range(len1 + 1)]

    for idx in range(1, len1 + 1):
        table[idx][0] = table[idx - 1][0] + deletion_cost

    for idx in range(1, len2 + 1):
        table[0][idx] = table[0][idx - 1] + insertion_cost

    for idx, c1 in enumerate(str1):
        for jdx, c2 in enumerate(str2):
            sub_cost = 0 if c1 == c2 else substitution_cost
            table[idx + 1][jdx + 1] = min(
                table[idx][jdx + 1] + deletion_cost,
                table[idx + 1][jdx] + insertion_cost,
                table[idx][jdx] + sub_cost
            )

    return table[len1][len2]


# Function to calculate the
# Jaro Similarity of two s
def jaro_winkler_similarity_python(s1, s2):
    # If the s are equal
    if s1 is None or s2 is None:
        return 0.0


    if (s1 == s2):
        return 1.0
 
    # Length of two s
    len1 = len(s1)
    len2 = len(s2)
 
    # Maximum distance upto which matching
    # is allowed
    max_dist = floor(max(len1, len2) / 2) - 1
 
    # Count of matches
    match = 0
 
    # Hash for matches
    hash_s1 = [0] * len(s1)
    hash_s2 = [0] * len(s2)
 
    # Traverse through the first
    for i in range(len1):
 
        # Check if there is any matches
        for j in range(max(0, i - max_dist),
                       min(len2, i + max_dist + 1)):
             
            # If there is a match
            if (s1[i] == s2[j] and hash_s2[j] == 0):
                hash_s1[i] = 1
                hash_s2[j] = 1
                match += 1
                break
 
    # If there is no match
    if (match == 0):
        return 0.0
 
    # Number of transpositions
    t = 0
    point = 0
 
    # Count number of occurrences
    # where two characters match but
    # there is a third matched character
    # in between the indices
    for i in range(len1):
        if (hash_s1[i]):
 
            # Find the next matched character
            # in second
            while (hash_s2[point] == 0):
                point += 1
 
            if (s1[i] != s2[point]):
                t += 1
            point += 1
    t = t//2

    '''
    print('====PYTHON====')
    print(f'n_matches: {match}')
    print(f'n_transpositions: {t}')
    print(f'len1: {len1}')
    print(f'len2: {len2}')
    '''
 
    sim = (match / len1 + match / len2 + (match - t) / match) / 3.0

    # Calculate the Jaro-Winkler similarity
    scaling_factor = 0.1
    max_prefix_length = 4
    prefix = 0
    for i in range(min(len1, len2, max_prefix_length)):
        if s1[i] == s2[i]:
            prefix += 1
        else:
            break
    
    return sim + prefix * scaling_factor * (1.0 - sim)


def jaccard_similarity_python(str1, str2):
    if str1 is None or str2 is None:
        return 0.0

    if str1 == str2:
        return 1.0

    len1 = len(str1)
    len2 = len(str2)

    if len1 == 0 or len2 == 0:
        return 0.0

    intersection = len(set(str1).intersection(str2))
    union = (len1 + len2) - intersection

    return float(intersection) / union



start = perf_counter()
for idx in range(N_ITERS):
    sim_python = jaro_winkler_similarity_python(x, y)
end = perf_counter()
print(f'Python Jaro Winkler time:          {end - start} seconds')
print(f'Python Jaro Winkler similarity:    {sim_python}\n')


start = perf_counter()
for idx in range(N_ITERS):
    sim_rust = jaro_winkler_similarity(x, y)
end = perf_counter()
print(f'Rust Jaro winkler Elapsed time:    {end - start} seconds')
print(f'Rust Jaro winkler similarity:      {sim_rust}\n')

print(81 * "=" + "\n")


str1_list = N_ITERS * [x]
str2_list = N_ITERS * [y]
start = perf_counter()
sim_rust = jaro_winkler_similarity_batched(str1_list, str2_list)
end = perf_counter()
print(f'Rust Jaro winkler Batched Elapsed time:    {end - start} seconds')
print(f'Rust Jaro winkler Batched similarity:      {sim_rust[0]}\n')

print(81 * "=" + "\n")

start = perf_counter()
for idx in range(N_ITERS):
    sim_rust_wlev = weighted_levenshtein_distance(
            x, 
            y, 
            insertion_cost=1, 
            deletion_cost=1,
            substitution_cost=1
            )
end = perf_counter()
print(f'Rust wlev Elapsed time:            {end - start} seconds')
print(f'Rust wlev distance:                {sim_rust_wlev}\n')


start = perf_counter()
for idx in range(N_ITERS):
    sim_python_wlev = weighted_levenshtein_python(x, y)
end = perf_counter()
print(f'Python wlev Elapsed time:          {end - start} seconds')
print(f'Python wlev distance:              {sim_python_wlev}\n')

print(81 * "=" + "\n")

start = perf_counter()
for idx in range(N_ITERS):
    sim_rust_jaccard = jaccard_similarity(x, y)
end = perf_counter()
print(f'Rust jaccard Elapsed time:         {end - start} seconds')
print(f'Rust jaccard distance:             {sim_rust_jaccard}\n')


start = perf_counter()
for idx in range(N_ITERS):
    sim_python_jaccard = jaccard_similarity_python(x, y)
end = perf_counter()
print(f'Python jaccard Elapsed time:       {end - start} seconds')
print(f'Python jaccard distance:           {sim_python_jaccard}\n')
