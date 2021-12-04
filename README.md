# [Advent of Code 2021](https://adventofcode.com/2021)

### Setup

I've never done one of these before, so I'm not sure what to expect.

Used `cargo` to create a new project, it initialized the git repo for me.  I
copied a bit of boilerplate command line argument handling and debug logging
from another project, just to be ready.

## Day 1 Developer Log

### First Star

Processing a data file. This looks pretty straightforward. For now, dropping
data file into a project directory, then opening and reading it at runtime.

The data processing looks like a great fit for rust Iterator combinators. By
wrapping the File reader in a BufReader, I now get automatic line parsing.
Then use `u32::parse` to convert each entry as it comes through the iterator.

I'm shortcutting the I/O and parse errors into a `panic!` for now. I'll come
back and clean that up later.

I find myself missing the `windows()` operator on slices, which is not
available on iterators.  Instead, I'm using a slightly more clumsy `fold()`.

The state I keep through each iteration of `fold()` is the accumulated count,
and prior value for comparison, as a tuple. Not bad at all.

### Second Star

Oh... I was afraid of this. I definitely don't want to re-read the file to do
this second round of processing (the iterator that read the file having been
consumed by the processing for the first star). I have two choices: calculate
both stars on the same pass through the iterator (probably storing the
look-back data in a `VecDeque`), or just loading all of the program data into
RAM as a `Vec`, and do two separate runs through the data. The first option is
much more efficient - effectively constant memory usage, improved cache
efficiency due to locality. The problem is that it's much less modular and
composable. I mean, I could make it more composable, but I'd be introducing
more logical cross-contamination between tasks by having to encode special
logic at the loop level like how much look-back we need, and how many results
we're accumulating. So in the interest of expediency, I'm refactoring to
collect all the data into a `Vec`, and iterating over it twice.

Besides, now that I'm operating on a `Vec`, I have access to slice methods like
the oh-so-cool `windows()` method. I do still need to keep around extra state
across iterations - the sum calculated from the previous triple. But this is
still pretty clean and readable.

So how am I handling the boundary case of the first iteration, where there was
no prior sum to compare against? Since the first iteration should never be
counted as higher than the one before it, I initialize the state variable for
the last sum to `std::u32::MAX`. By definition, nothing is higher than that, so
we see the right behavior. This spares me having another state variable just
for tracking whether we're in the first iteration or not, and paying the cost
of that test every subsequent iteration.

## Day 2 Developer Log

### First Star

More complex data file - this looks fun. Going to use a simple split-on-space
scheme, parsing the `&str` on the right into a number, and converting supported
left-side `&str`s into a particular enum value.

Looking a little deeper into what I'm doing with this data, rather than a
straight-up enum-value-for-each-input-value, I'm going to map `up` and `down`
to the same enum value (`Depth`), and negate the argument to it in the `up`
case.

By implementing the TryFrom trait on my enum, I get very simple conversion from
a line of text into a value in my enumeration, with pretty clean error handling
should there be a problem with my logic or the input.

Straightforward application of iterator combinators to accumulate the x and y
from the `Travel::Forward(dx)` and `Travel::Depth(dy)`, multiply them together
to calculate the "travel distance" (net-depth times forward distance), et voilà!

### Second Star

Oh my, this is almost embarassingly simple after doing up the first one.

Augment the accumulator in my `fold()` from `(x, y)` to `(x, y, aim)`, update
the match cases accordingly, and the result falls perfectly out. I actually
didn't trust that it was truly that easy so I went and double-checked
everything. Perfect.

## Day 3 Developer Log

Reading a reddit post in `r/rust` about AoC I saw someone who was using the
sample data provided for each day as a test to validate their solution. That
seems like an amazing idea, so I'm adding in tests for the previous days, and
I intend to follow that pattern going forward. Also, super happy with how I
divided up my functions because it made the tests really simple.

### First Star

Well, this one threw me off, largely because at first I missed the fact that
the the actual dataset didn't use the same number of bits as the sample
dataset. So after I got the test working, I had to go back and refactor my
algorithm to accommodate an arbitrary number of bits to operate on.

### Second Star

Ok, gotta admit. This one took me a long time to really grok what was being
asked. Once I figured it out, it took me a few iterations to get my solution
into a form I was happy with, performance- and clarity-wise.

My first pass at a solution for this relied on sorting. Rust's `sort_by_key()`
and `partition_point()` along with the slicing semantics, made a recursive
algorithm pretty efficient.

Once I saw that with each recursion I was 'locking' a new bit in place, I
realized that there's an O(n * log(n)) solution (as opposed to the
O(n log^2(n)) solution I'd originally gone with), and I'm doing something very
similar to what I did in the first star, but instead of counting the bit among
all the array entries, I'm counting only among the ones with the pattern of
bits that were 'locked in' previously.

## Day 4 Developer Log

### First Star

In many ways this is a lot like the tic-tac-toe game I'd made in rust
previously. This biggest wrinkle is multiple boards and finding the
first highest scoring board among multiple.

Oops... spoke too soon. Hmmm... When I'm doing line-and-space splitting for
parsing the board numbers, a String is being produced in the iterator, and then
I'm calling `split(' ')` on that String, which produces `&str`, but that `&str`
references a String that's being dropped during this iteration. Unclear why
convert the `&str` to an owned `String` doesn't resolve the issue
(`s.to_owned()) but `collect()`ing the iterator before this step and then
iterating over `&String`s totally fixes it for me.

Also, got my first taste of the `inspect()` iterator combinator. Very nice for
debugging into the iterator pipeline.

### Second Star

This problem looks very easy at first. Instead of getting the max score from
each round, you get the min score. But you need to keep going in rounds because
if a board wins in a later round, then its score should be even lower. Oh...
but not necessarily, because the product of the remaining numbers could still
be higher than that of an earlier completeing board. And how many rounds do you
go? Can you choose not to call bingo on a board that has completed? If that's
the case, the lowest score possible is 0, because you just let all the numbers
get called. Pretty sure that's not what's intended.

So really all I need is keep the lowest score for this round, replacing the
lowest score from the previous round, until all the boards have won.

In the course of implementing this, I did some refactoring to create a cheap
test for whether a board has already won, so that I can stop iteration as
soon as all boards have won, rather than continuing to call numbers after
all the boards have already finished.
