# The Rust Book's History
## Introduction

On February 28th, 2018 Steve Klabnik wrote out some thoughts on his experience with writing the Rust
Book from it's pre-1.0 version, to the 1.0 release, all the way up to the second edition. It's a
long bit of history. He wrote it out on [this reddit thread](https://www.reddit.com/r/rust/comments/80qp3x/this_week_in_rust_223/duyohad/?context=1#duxuzw5) of This Week In Rust's 223rd issue. The comment has been
reproduced in full below:

## Steve Klabnik
To be clear, it's not *done* yet, but we're in the last phases. Been reviewing the final page
layout, stuff like that.

> 2 or 3 years of work

So, some history. I learned about Rust on December 21, 2012, when
[0.5 came out](https://mail.mozilla.org/pipermail/rust-dev/2012-December/002787.html). I thought it
looked neat, but was a bit confused by [the tutorial](https://doc.rust-lang.org/0.5/tutorial.html).

I had been writing another book, and had wanted to try out a new markdown -> PDF toolchain. So I
decided to write down what I learned, as I learned it, and make my own tutorial. This would help me
remember Rust better, and let me give the toolchain a try, and create another tutorial. So the
[next day I wrote 5 chapters](https://github.com/steveklabnik/rust_for_rubyists/commits/master?after=1ca0d9bb2e3e8beb6a13c60791fe13b8a768d790+314) of what I called "Rust for Rubyists", because I love alliteration, and
wanted to be clear about where I was coming from.

EDIT: A friend pointed out [this tweet](https://twitter.com/steveklabnik/status/282663509538508802)
from that day too, which is pretty hilarious looking back.

On the 27th, I [sent in my first PR to Rust](https://github.com/rust-lang/rust/pull/4305). It was
rejected because in those days, you sent PRs to `incoming`, not `master`. Oops! This was before
GitHub let you re-target PRs, so I had to submit a
[second one](https://github.com/rust-lang/rust/pull/4308). That's right, my first PR was rejected,
but I'm on the core team today. It takes some perseverance. :)

I sent in 34 PRs in 2013, ending in October. It ended because, well, I got a new job, and moved to
San Francisco. I say "I got a new job" like it's simple, but it was more complicated than that: I
had actually had an offer for a contract to do docs for Rust, but I had misunderstood what that
meant. See, Mozilla really likes to hire on contract first and then give you a real job later. I
saw "here's money for a few month's work" and kept looking for a real job. I got one pretty quickly.
In the end, that contract never happened. I felt really bad, but it was just a communication mix-up
on all sides.

Thanks to the new job, I didn't have a ton of time for PRs, but I
[laid out my suggestions for what the docs should be like](https://air.mozilla.org/rust-meetup-december-2013/)
at the SF meetup in December of 2013.

In April of 2014, for Reasons, I emailed brson and asked him if Mozilla might reconsider. By June,
we had signed, and
[made it public](http://words.steveklabnik.com/rusts-documentation-is-about-to-drastically-improve).
I also started submitting PRs again; an 8 month period of nothing, my longest ever (and since) since
starting working on Rust.

One of those first PRs was [this one](https://github.com/rust-lang/rust/pull/15131), which changed
"the tutorial" to "the guide". Does that warning box look familiar? And the reasons for doing it
like this? I am at least consistent in my reasoning.
[Here's a humble little table of contents](https://github.com/rust-lang/rust/pull/15211/files).
See, at first, all there was was the tutorial. But we started adding guides on various things.
[Here's 0.9's docs](https://doc.rust-lang.org/0.9/) for example, you can see there's the Reference,
Tutorial, Guides, crates, tooling, and FAQs. By the way, check out the
[`rustpkg` guide](https://doc.rust-lang.org/0.9/rustpkg.html) sometime. We actually had a few
different build systems before Cargo. We even had one named Cargo!

So, on December 2, 2014, I opened [this PR](https://github.com/rust-lang/rust/pull/19461), merging
the guides together into "the Rust programming language", aka the book.

> Basically, the new "book" as I'm calling it looks like this:
>
> * Part I: Basics <- this is the first half of the current Guide. Linear.
> * Part II: Intermediate <- this is some of the current Guide, with the invididual Guides replacing
chapters where relevant. Can be read in any order.
> * Part III: Adavanced <- the most advanced of the previous Guides. Can be read in any order.
>
> Conceivably, this could even fix japaric/rust-by-example#278 by making Rust by example a part IV
or something. While Rust By Example doesn't currently use Rustbook, it could.

That last bit is foreshadowing a few years...

The book got improved a lot, including a frenetic set of updates right before 1.0, but the next
major chapter happens in August of 2016.
[I announce the second edition](http://words.steveklabnik.com/whats-new-with-the-rust-programming-language)
, with an initial six draft chapters:

> In some ways, this mirrors the compiler itself, which accrued a massive amount of technical debt
until 1.0. And so, the story since 1.0 has largely about been paying off this debt; culminating in
MIR. By the same token, since 1.0, a lot of people have read the book, and a lot of people have
tried to learn Rust. I now know so much more about how people go about doing so, and so am much,
much better situated to write something that’ll be great for them.

This includes, of course, bringing Carol on as a co-author, significantly increasing the book's
quality. There's no way that I'd have this done, nor have it be as good, without her. The second
edition is just as much hers as it is mine.

So uh, yeah. 18 months.

Books are hard.
