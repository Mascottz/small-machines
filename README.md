# Small machines

Three tools, one file each; nothing else needed. no server, no packages, no account, no network. each one exists as itself, and the whole repo is the whole product.

## The machines

`posters.rs` · rust, std only; a generative poster press. every seed is a calm poster from the same family; one loud shape, a field of quiet marks, type sitting low.

```bash
rustc -O posters.rs -o posters
./posters 42 > poster-42.svg
```

`tide.jl` · julia, stdlib only; the day as a rising line of hours in the terminal, past ones full, the current one partial, the rest still water.

```bash
julia tide.jl
```

`breath.html` · one file, zero dependencies; a breathing instrument. open it from disk and the circle keeps the time; 4·4·6 by default, box breathing one click away.

## The rules it was built under

- one file per machine; if a machine ever needs a second file, it is a different project now
- no dependencies beyond the language's own standard library
- no network, no accounts, no clocks that phone home; tide reads the local clock and nothing else
- money never appears, so cents stay out of it, but the calm does not
- headers and paragraphs start with a capital; the rest stays casual

## Why this exists

Most software is a promise that someone else's computer will behave. these three are objects, not services; compile one, open one, run one, and it is done. a portfolio of small perfect things, next to the big loud ones.
