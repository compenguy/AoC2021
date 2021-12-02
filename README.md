# [Advent of Code 2021](https://adventofcode.com/2021)

## Day 1 Developer Log

### Setup

I've never done one of these before, so I'm not sure what to expect.

Used `cargo` to create a new project, it initialized the git repo for me.  I
copied a bit of boilerplate command line argument handling and debug logging
from another project, just to be ready.

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
