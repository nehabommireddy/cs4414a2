# CS4414 HW 2 - Fast edit distances

You should complete the following questions in your final submission:

## Step 0: Build modes

1. For the dictionary `examples/popular.txt`, how long does it take to
   run the code in the `debug` compilation mode?  What about the
   `release` mode?

Using the naive implementation, the computation time for `popular.txt` was:
Debug: 99.316334992 seconds
Release: 3.822227307 seconds

2. Based on the relative sizes of the dictionaries, estimate how long
   you think it would take to run in the two modes for the
   `examples/enable1.txt` dictionary.  Test your hypothesis in release
   mode; how long does it actually take to run?

The `popular.txt` dictionary contains 25,322 words, while `enable1.txt` contains 172,823 words. The ratio of the dictionary sizes is: 172,823 / 25,322 ≈ 6.83. Since the program compares pairs of words, the amount of computation grows approximately quadratically with the number of words. Therefore, I estimated the runtime ratio as: 6.83² ≈ 46.7.

Based on the `popular.txt` runtimes, my estimated runtimes for `enable1.txt` were:

- Estimated debug runtime: 99.316334992 × 46.7 ≈ 4,638.1 seconds
- Estimated release runtime: 3.822227307 × 46.7 ≈ 178.5 seconds
- Actual release runtime: 197.674267333 seconds


## Step 1: Blocking

1. What is the estimated memory footprint for the two dictionaries
   (`popular.txt` and `enable1.txt`)?  Include the storage for the
   `String` metadata in your accounting (don't worry about storage for
   the allocator's data structures).

A Rust `String` contains 24 bytes of metadata on a 64-bit system: 8 bytes for the pointer, 8 bytes for the length, and 8 bytes for the capacity. Since the dictionary contains ASCII characters, each character requires 1 byte.

For `popular.txt`:
- Number of strings: 25,322
- Character storage: 185,196 bytes
- String metadata: 25,322 × 24 = 607,728 bytes
- Total estimated memory footprint: 792,924 bytes

For `enable1.txt`:
- Number of strings: 172,823
- Character storage: 1,570,540 bytes
- String metadata: 172,823 × 24 = 4,147,752 bytes
- Total estimated memory footprint: 5,718,292 bytes

2. Complete the code for the blocked variant.  You should see a speed
   difference that is noticeable, but not enormous.  What difference
   do you see?

   Using the blocked implementation (BSIZE = 500) on `popular.txt` in release mode:
   Naive:   3.822227307 seconds
   Blocked: 2.976012848 seconds

   This is about a 22% speedup. Blocking helps because it keeps a small part of the dictionary in the cache while it is being reused, instead of going through the entire array each time. The improvement is not huge because blocking only changes the order we access the `String` data. Each `dist()` call still has to follow a pointer to find the actual characters stored elsewhere in memory, where blocking does not make those characters stored closer together.

## Step 2: Removing indirection

1. What is the speed difference compared to the method in step 2?

Packing was faster than blocking for both dictionaries. For `popular.txt`, the runtime decreased from 2.976 seconds to 0.997 seconds, making it about 2.98x faster (66.5% reduction in runtime). Similarly, for `enable1.txt`, it decreased from 195.241 seconds to 46.522 seconds, making it about 4.20x faster (76.2% reduction in runtime).

Overall, packing gave a much bigger improvement than blocking. This is likely because packing removes the extra pointer lookups needed to access a `String`'s heap-allocated data. It also gives the compiler a fixed loop size (`0..WSIZE`), allowing it to optimize the loop more efficiently through unrolling and vectorization. Blocking alone does not provide these same benefits.

2. The longest word in `enable1.txt` is 28 characters, but most are
   shorter.  If you write your code to reserve one byte for the word
   length at the beginning, what type of performance improvement do
   you see?

Surprisingly, adding a length byte actually made the program much slower. For `popular.txt`, the runtime increased from 0.992 seconds to 3.567 seconds, making it about 3.6x slower. Similarly, for `enable1.txt`, it increased from 46.318 seconds to 179.225 seconds, making it about 3.87x slower.

This is likely because the compiler can optimize a loop with a fixed size much better, such as by unrolling it. However, when the loop length depends on a value determined at runtime, the compiler has fewer opportunities to optimize it. So even though adding a length byte means we compare fewer characters on average, the loss in compiler optimization ends up making the program slower overall.

## Step 3: Packed representation

What do you see?  Is your version any faster than the
method you explored in Step 2?

SWAR was significantly slower than our Step 2 implementation for both dictionaries. For `popular.txt`, SWAR took 3.669 seconds compared to 0.997 seconds for Step 2, making it about 3.68x slower. Similarly, for `enable1.txt`, SWAR took 171.284 seconds compared to 46.522 seconds, also making it about 3.68x slower. This is likely because our Step 2 implementation uses a fixed loop size (`0..WSIZE`), which the compiler can optimize efficiently through unrolling and vectorization. Although SWAR also uses a fixed loop size (`0..6`) and processes multiple characters at once, each iteration requires extra operations like XOR, addition, masking, and `count_ones()`. These extra operations seem to outweigh the benefit of having fewer iterations.

Overall, this shows that a more complicated implementation does not necessarily mean better performance, especially when the compiler can already optimize a simpler approach really well.

## Step 4: Speed demon

Describe the steps that you took to get to your final optimized
version!

Starting from our Step 3 SWAR implementation, we made two main changes:

1. **Used symmetry to reduce comparisons.** Since `dist(w1, w2) == dist(w2, w1)` and `dist(w, w) == 0`, we only need to calculate each pair of words once. Instead of comparing every word against every other word, we only compute the upper triangle of the distance matrix and add the result to both words' totals. This reduces the number of `dist()` calls by roughly half.

2. **Packed 10 characters into a `u64` instead of 5 into a `u32`.** Our original SWAR implementation used 6 bits per character, meaning we needed 6 packed words to store a 28-character string. By switching to `u64`, we can fit 10 characters per word, reducing the number of packed words from 6 to 3. This means fewer XOR, addition, masking, and `count_ones()` operations per comparison.

We also used a `const fn` to generate the add and mask constants instead of manually extending the original 32-bit constants, since simply doubling them would misalign the bits. To make sure everything still worked correctly, we tested our packed `dist()` against `basic_str::dist()` using several word pairs, including one that crosses the 10-character boundary.

The results showed a pretty significant improvement. For `popular.txt`, the runtime decreased from 3.669 seconds to 1.033 seconds, making it about 3.55x faster. Similarly, for `enable1.txt`, it decreased from 171.284 seconds to 48.340 seconds, making it about 3.54x faster.

Overall, both dictionaries saw roughly a 3.5x speedup. This makes sense because we reduced the number of comparisons through symmetry while also cutting the number of packed-word operations per comparison in half.