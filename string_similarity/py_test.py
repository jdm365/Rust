import numpy as np
from string_similarity import jaro_winkler, jaro_winkler_vector, jaro_winkler_simd
from jarowinkler import jarowinkler_similarity 
from math import floor
from time import perf_counter


N_ITERS = 1000000

#x = 'abcdefghijklmnopqrstuvw'
#y = 'zyxwvuasdflkjasdhgfedcb'

x = 'abcdefasdlk;fas;dlkjfkjl;sadfkjl;sadfklj;ka;jlfsdghijklmnopqrstuvwxyzaklsjdflaksjjdfkjlasdklfj'
y = 'zyxda;slkfdakjl;k;ljdfkjl;asdlk;jdfsakl;jadfskl;jfdsalk;jwvuasdflkjasdhgfedcbaaskjldfalskdfkjlasdkfljaasdklf;lkjasdkl;jfklj;asdkflj;asklj;dk;ljfakl;jslk'

# Function to calculate the
# Jaro Similarity of two s
def jaro_distance(s1, s2):
     
    # If the s are equal
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
 
    # Return the Jaro Similarity
    return (match/ len1 + match / len2 +
           (match - t) / match)/ 3.0


'''
start = perf_counter()
for idx in range(N_ITERS):
    sim_python = jaro_distance(x, y)
end = perf_counter()
print(f'Pure Python Elapsed time: {end - start} seconds')
'''



start = perf_counter()
for idx in range(N_ITERS):
    sim_rust = jaro_winkler(x, y)
end = perf_counter()
print(f'Rust Bitmap Elapsed time: {end - start} seconds')


start = perf_counter()
for idx in range(N_ITERS):
    sim_rust = jaro_winkler_vector(x, y)
end = perf_counter()
print(f'Rust Vector Elapsed time: {end - start} seconds')


start = perf_counter()
for idx in range(N_ITERS):
    sim_rust = jaro_winkler_vector(x, y)
end = perf_counter()
print(f'Rust SIMD Elapsed time: {end - start} seconds')


start = perf_counter()
for idx in range(N_ITERS):
    sim_cpp = jarowinkler_similarity(x, y)
end = perf_counter()
print(f'(C++/Cython) Library Elapsed time: {end - start} seconds')




np.testing.assert_approx_equal(sim_cpp, sim_rust, err_msg=f'Results do not match - rust: {sim_rust}, cpp: {sim_cpp}')
