# Small machines

Five tools, one file each; nothing else needed. no server, no packages, no account, no network. each one exists as itself, and the whole repo is the whole product.

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

`lumen.rs` · rust, std only; the palette auditor. every foreground and background pair scored for the wcag contrast ratio in four visions; as-is, protanopia, deuteranopia, tritanopia. failing pairs get a walked fix in the same breath.

```bash
rustc -O lumen.rs -o lumen
./lumen                # audit the built-in calm palettes
./lumen #0D1A15 #C98E5F   # audit your own
./lumen --selftest
```

`night.jl` · julia, stdlib only; the sky, computed not fetched. moon phase and altitude, the planets above your horizon, and the one bright thing worth stepping outside for, from mean keplerian elements and a truncated moon theory; good to about a degree, and honest about it.

```bash
julia night.jl                    # tonight, from lagos
julia night.jl 2026-12-14 6.45 3.39  # any date, any where
julia night.jl --selftest
```

## The rules it was built under

- one file per machine; if a machine ever needs a second file, it is a different project now
- no dependencies beyond the language's own standard library
- no network, no accounts, no clocks that phone home; tide and night read the local clock and nothing else
- machines that compute something carry a `--selftest` flag with pinned checks; lumen and night prove their math on the spot
- money never appears, so cents stay out of it, but the calm does not
- headers and paragraphs start with a capital; the rest stays casual

## Why this exists

Most software is a promise that someone else's computer will behave. these three are objects, not services; compile one, open one, run one, and it is done. a portfolio of small perfect things, next to the big loud ones.
