# commenter
A couple of simple applications that all do exactly the same thing, to benchmark different languages, Web frameworks and ORMs.

## Evaluating
Please see each language directory for information on how to run.
A standardised test consisting of `oha http://localhost:(PORT) -n 500000` was used.

## Test parameters:
The tests in `oha_results.txt` were done with Rust in release mode and `go run`,
on an Intel Core i7-10750H 6-core, 12-thread laptop processor, running at 5 GHz,
on Arch Linux x86_64 - Sat 21 May 2022.

Rust was about three times faster (23 kreqs/sec vs 60 kreqs/sec). (And the language is better. But I'm biased. Bleh.)

Also thanks [Hannah](https://twitter.com/ravenslofty) for making me realise I put Rust in debug mode, skewing the results in the original commit.