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

## Day 5 Developer Log

### First Star

So, Rust got in my way a little bit here. On two separate occasions I tried
using a feature that's only available in nightly, missing the little warning
info box underneath the function name. And finally, there's no way to generate
a `Range` that counts backwards, so I had to jump through some hoops with
running the range through the `rev()` iterator combinator, which is less than
ideal from a performance and memory perspective. Even worse was the first
indication I found that `Range`s don't count backwards was when my tests ran
and produced a sum less than the example.

I also started implementing this thinking that it meant that all endpoint pairs
formed only horizontal or vertical lines, so my logic did weird things with the
45-degree lines. Once I realized that, I had a pretty solid idea what the
second star was going to be, and fixing that logic bug pretty much handed me
the solution for the second star.

I like my solution for scoring the field, too. I iterate over all points in all
the lines, building up a hashmap tying points to scores. Getting the final
count then was super easy - barely an inconvenience. Iterate over values, apply
a combinator that filters for values greater than 2, then apply the `count()`
combinator. Also, the `inspect()` combinator is so cool for debugging that
there's no chance I'll forget about it in the future.

### Second Star

Yep - counting up all the overlapping lines, not just the horizontal or
vertical ones. I just duplicated my top-level logic for the first star,
removing the filter selecting for just the horizontal and vertical lines.

## Day 6 Developer Log

### First Star

They said "simulate", so I wrote a simulator. I thought I was being clever
implementing this as a delta queue. An entry in the queue for each fish.

But my delta queue got overwhelmed rather quickly with all the insertions we
were doing. Simulating for 80 days got a bit out of hand.

I thought "I should fold together all the fish with the same count" at which
point I quickly reduced the problem to an array of buckets for fish of each
"age". A count of how many fish for each age, and we're done.

### Second Star

Running the simulation for 256 days was pretty trivial at this point. No longer
iterating over each fish with each day, and doing a bunch of insertions,
there's only 9 values to update, one for each quantity of time remaining (0-8).

After I submitted my answer, I came up with a minor refinement, using a
`VecDeque` to handle the rotation of ages each day. Pop the reproducing ones
off the front, push that count onto the back as the new ones, and add that
same number into slot 6 in the array for re-introducing the original ones. Now
the `tick()` function is really clean and readable.

## Day 7 Developer Log

### First Star

First step was to come up with a scoring function that takes the data set and a
target point, and assigns a score to that solution. Next, characterize the
function output. Moving from low to high target, it declines to the minimum,
then rises.

So in my score optimizer, I take a sample of values, and find the set of values
that have a negative delta then a positive delta (e.g. \./), because I know
that range contains the optimum value.

I could do this as a recursive process, continually sampling, and then "zooming
in", but these problems are taking longer each day, so rather than optimize my
code further, I'm just going to iterate over all the values in this subset and
keep the one with the best score.

### Second Star

Oh, neat.  Write a new scoring function, later rinse repeat.

## Day 8 Developer Log

### First Star

Hmph. Today's problem looks a bit scary. Not this first part, but I have a
pretty solid guess what they'll have us doing for the second star.

This step is pretty easy, and can be trivially written with iterator
combinators.

### Second Star

Yep. Ugh. Well, ok - I'll use HashSets and HashMaps and do set operations to...
nope. HashSet can't be a key to a HashMap. I suppose that makes sense. Hmmm...

Ok. I'll skip the HashSet to track which wiress are active, and I'll model it
as just bits in a bitset. I can implement set operations on top of my wrapper
type and I can reuse pretty much all of my deductive logic from when I tried
using the HashSet. That reads a whole lot nicer, and probably performs a good
bit better with lower memory usage.

I'm fairly happy with my final solution. I should probably remove the digits
I've already figured out from my `digits` HashSet (a cheap operation that
keeps me from continually re-iterating over digits I already know to discover
the next known one), but as I said yesterday, these problems are taking more
time.

## Day 9 Developer Log

### First Star

Looks pretty simple.

### Second Star

Also not bad. I did need to go back and make sure my recursive basin expansion
function didn't attempt to re-evaluate points it had already added, but that
was a fairly obvious and simple fix.  Otherwise, easy peasy.

## Day 10 Developer Log

### First Star

Prime opportunity for a stack. I've done parsing like this before.

### Second Star

Well, we've already got the data for the completions in our stack from the
part. We just map them to their corresponding close tags, and append it to
the original string, right? Ooops... play them back in reverse.

Also, very fun convert from the `Option<mismatched byte>` that I used in the
first part, to a `Result<completed string, mismatched byte>`. Made it very easy
to reuse the logic for the first part, but to filter out bad strings, instead
of filtering for them. I'm really happy about how that turned out.

## Day 11 Developer Log

### First Star

State tracking was a bit tricky with this one. Had to keep track of which ones
I'd already recorded as having flashed. More than that, I also needed to keep
track of which ones that flashed I'd already updated the neighbors for, and
which ones I still needed to. Once I had the state tracking in place, the rest
just came together.

### Second Star

Great, now we just run the steps until the number of lights flashed in that
step equals the size of our dataset. Super easy, barely an inconvenience.

## Day 12 Developer Log

### First Star

Maze solver with a twist. I initially started out modelling this with nodes and
edges, but I decided to go back and do an adjacency list. The special handling
around `start` and `end` nodes, and "minor" nodes (lowercase nodes that can only
be visited once) added some complexity, but not really difficulty.

Also got to implement FromIterator on my Maze struct for the parsing, which was
new and kinda cool.

### Second Star

And another twist. Add a parameter for whether we've already revisited a
`minor` node or not, and keep a HashSet of completed paths so that I don't
accidentally recount a solution. Also added a bit more special logic around
`start` and `end` to prevent them from being treated as a `minor` node for
revisit purposes. Essentially, all adjacencies get added in as two-way
adjacencies, except for `start` and `end` -- `start` is never on the target end
of an adjacency and `end` never has any targets in its adjacencies.

## Day 13 Developer Log

### First Star

Only get the first iteration? It looks like I'll be able to have a working
solution for part 2 if my part 1 passes the tests.

Reflecting and deduping points? This sounds like a job for a `HashSet<(x, y)>`.

Super easy, barely an inconvenience. Except that I only had time to start it
then didn't have time again for several days. But otherwise, yeah, neat.

### Second Star

Oh, not the count, but actually render. Well, I'll just get the bounding box for
the points by using the little-used `max_by_key()` method on `HashSet`s, and
iterate over all the points printing out ` ` if the point doesn't exist in the
`HashSet` of points, and printing out `#` if it does.
