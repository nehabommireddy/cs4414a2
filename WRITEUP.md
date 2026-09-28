# CS4414 HW 2 - Fast edit distances

You should complete the following questions in your final submission:

## Step 0: Build modes

1. For the dictionary `examples/popular.txt`, how long does it take to
   run the code in the `debug` compilation mode?  What about the
   `release` mode?

Using the naive implementation, the computation time for `popular.txt` was:
Debug: 97.746620311 seconds
Release: 4.13882222 seconds

2. Based on the relative sizes of the dictionaries, estimate how long
   you think it would take to run in the two modes for the
   `examples/enable1.txt` dictionary.  Test your hypothesis in release
   mode; how long does it actually take to run?

The `popular.txt` dictionary contains 25,322 words, while `enable1.txt` contains 172,823 words. The ratio of the dictionary sizes is: 172,823 / 25,322 ≈ 6.83. Since the program compares pairs of words, the amount of computation grows approximately quadratically with the number of words. Therefore, I estimated the runtime ratio as: 6.83² ≈ 46.7.

Based on the `popular.txt` runtimes, my estimated runtimes for `enable1.txt` were:

- Estimated debug runtime: 97.746620311 × 46.7 ≈ 4,565.2 seconds
- Estimated release runtime: 4.13882222 × 46.7 ≈ 193.3 seconds
- Actual release runtime: 215.616952767 seconds


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
   Naive:   214.525513361 seconds
   Blocked: 149.200662908 seconds

   This is about a 30% speedup. Blocking helps because it keeps a small part of the dictionary in the cache while it is being reused, instead of going through the entire array each time. The improvement is not huge because blocking only changes the order we access the `String` data. Each `dist()` call still has to follow a pointer to find the actual characters stored elsewhere in memory where blocking does not make those characters stored closer together.

## Step 2: Removing indirection

1. What is the speed difference compared to the method in step 2?

Step 2 was faster than Step 1 on both dictionaries. For `popular.txt`, the runtime decreased from 2.63 seconds to 1.00 second, giving a 2.63× speedup (61.92% faster). For `enable1.txt`, the runtime decreased from 149.56 seconds to 46.33 seconds, giving a 3.23× speedup (69.02% faster).

2. The longest word in `enable1.txt` is 28 characters, but most are
   shorter.  If you write your code to reserve one byte for the word
   length at the beginning, what type of performance improvement do
   you see?
Reserving one byte for the word length did not improve performance in our implementation. For popular.txt, the runtime increased from 1.00 seconds to 3.62 seconds, making it about 3.61× slower. For enable1.txt, the runtime increased from 46.33 seconds to 181.57 seconds, making it about 3.92× slower. Thus, the length byte resulted in a performance decrease rather than an improvement.

## Step 3: Packed representation

What do you see?  Is your version any faster than the
method you explored in Step 2?

## Step 4: Speed demon

Describe the steps that you took to get to your final optimized
version!
