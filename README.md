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
