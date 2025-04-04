# schedcalc

A reverse mixing algorithm for the game Schedule 1. This game has a grip on me and I felt a deep need to make this.

Of course, the website does exist, HOWEVER parts of the algorithm are behind a paywall! Not sure why we would do this, doesn't seem very fun to me.
Adding features should be easy, here are some I plan to add soon.

- Website if I'm feeling snazzy
- Optimizations (Built in Rust so it's already pretty speedy, but I do a disgusting amount of cloning, escape-hatching, and vec comparisons)

# Features

Features are quick to produce, so expect more if I think of them!

- Easy to use console interface, just run and follow the instructions!
- Discovers shortest paths to desired effects with desired ingredients.
- Displays the cost and sell price multiplier of every mix discovered.
- Search for highest sell multiplier for desired ingredients.

# Technical Details

Implemented a BFS algorithm to find paths. Adding features should be strait-forward for anyone who has their own ideas. Please, make a pull request or fork if you do! I love free, good tools.

It might be optimal to generate an actual graph one time, and then apply graph algorithms to it. I currently BFS on generation of nodes, so some tricks like meet in the middle are not possible
with the current implementation. Also, a pre-generated graph in memory would make this *incredibly fast* for repeat queries.

# Build, Usage

This is now ran as an interactable app in the command line.
Build with `cargo build --release` or ideally right now just hit `cargo run`.

Follow instructions and commands after!

# Rule Problems

So I initially intended to build this entirely off the rules listed on the only website that does this, however a good chunk of them are just completely wrong. I believe I have found the correct
implementation of most, but if you find a bad mix **PLEASE TELL ME I WILL FIX IT!**