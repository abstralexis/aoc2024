# aoc2024
I'm doing this as part of a University leaderboard, so the code may look very rushed. I like doing solutions that are
nice and organised but that may not happen this year! I was contemplating using Go this year to learn it, but I just 
could not wrap my head around the tooling. Most of these solutions will be in Rust, or Java if I'm feeling particularly
daring.
## Why have I done X?
Because I wanted to. I added `rayon` to dependencies in case I want to make things faster, and I might
add `tokio` too.
## Log
### Day 1
#### Part 1: Differences
I found this to be the hardest part of the day, simply because I had to write the parsing and figure
out where Rust likes the txt file to be. It also had a fair few instructions, and missing out sorting
on my first run definitely wasn't the best idea. 
#### Part 2: Bad Handwriting
So this Chief Historian has history teacher handwriting. Should've expected that. This task was to
make a hash map of one with counts, and make a sum of all the ones in the left list that appear in 
the right list, with each left entry multiplied by the counts in the right. `itertools` was great
for this, using the `counts` function. Annoyingly, it doesn't have implementations for the `rayon` 
parallel iterator types, but the performance was nice anyway. I got this one in the first run, which 
is nice. Early-ish completion today!
### Day 2
#### Part 1: Chernobyl Number 2
The elves are horrible at managing the control rods! Calculate how many rads Santa will receive.
Fun problem, Rust definitely slowed me down in places. It is helping me warm my hands up this 
winter though, so I can't complain much.
#### Part 2: Simply ignoring some of the problems and silencing those who say our reactor is unsafe
This was a harder part of the problem - I tried several attempts at an odd sliding window of sorts
for only checking permutations by removal when I really needed to, then I realised I am using Rust
and it will run fast anyway. I just checked all permutations manually for each entry and it worked
somehow. Unrelated, Rudolph's nose is now going green and he has an extra antler.
### Day 3
#### Part 1: Someone left in a strcpy vulnerability
So the memory is corrupted. It must have been written in C. The first part was entertaining to do. 
I had to remember regex that I haven't done in years, and somehow got it working.
#### Part 2: Do Do Do Do Do Do
So apparently we have these instructions that also need to run so we don't end up adding too much.
This part sent me nuts. Regex matching between a string predicate A and a string predicate B, where
it only matches up to the first occurrence of B, is a pain. I had to learn lookbehind wizardry, and
when I came to putting it into code i realised Rust's regex library doesn't support it. I had to 
remove them with an editor online and then use that as the puzzle input. That worked.
### Day 4
Missed. Will be grinding 5 early morning out of order!
#### Part 1: Get out of my way, wordsearch elf
Okay, I can see why you are struggling on this one. I first tried a sliding window approach, but i seemed to be getting almost double the amount i should have. I checked my approach tens of times and
got nowhere closer to fixing it. I ended up goig for the technique of getting all the horizontals,
verticals, and diagonals and using a 2D sliding window to search for all occurrences of `XMAS` or 
`SAMX`.
#### Part 2: RTFM
So I did all that work just to realise I didn't do the rules correct. Fine.
This part I found to be a lot easier. Somehow, I was able to do a sliding window just fine, maybe
because this one was only 3x3 and not 4x4. I got each window, and checked the diagonals using 
very similar logic to my first Part 1 approach. I got this one right first try! Maybe I was not 
doing the vertical or horizontal checks right? I might have been checking each one for `XMAS` **and** 
`SAMX`, rather than `XMAS` **or** `SAMX`. That might have been my issue. No matter, I'm top 5 in
the private leaderboard (for now, I shall soon be overtaken by Sergey I just know it).
### Day 5
#### Part 1: Corrupted updates
These guys can't make a non-broken system can they?
This one was fairly easy - parse the rules, and for each parsed update I check to see if it breaks
any of the rules. I then sum the middle number of each, for some reason.
#### Part 2: Oh, but we also want the corrupted ones fixed
I can't catch a break with these elves.
This one was also fairly easy, we just do what's essentially a bubble sort on each update, where the 
indices to swap are the found locations of the update pages that break rules. Do that enough times
until each and every one are sorted, and then it's all done.
### Day 6
```rs
todo!();
```
### Year Summary
```rs
todo!();
```
