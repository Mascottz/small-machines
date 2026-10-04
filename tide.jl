# small machines; tide. the day as a dial in the terminal, one file,
# stdlib only; it exists as itself.
#
#   julia tide.jl
#
# prints the day as a rising line of hours, past ones full, the current
# one partial, the rest still water. no network, no accounts, no noise.

using Dates

function cell(frac::Float64)
    frac <= 0 && return " "
    frac >= 1 && return "█"
    return ["▁", "▂", "▃", "▄", "▅", "▆", "▇"][max(1, min(7, ceil(Int, frac * 7)))]
end

function main()
    t = now()
    h, m = hour(t), minute(t)
    day_frac = (h * 60 + m) / 1440.0

    cells = String[]
    for i in 0:23
        if i < h
            push!(cells, "█")
        elseif i == h
            push!(cells, cell(m / 60))
        else
            push!(cells, " ")
        end
    end

    println()
    println("  small machines · tide")
    println()
    println("  |" * join(cells, "") * "|")
    println("   0                          24")
    println()
    println("  hour $(lpad(string(h), 2, "0")):$(lpad(string(m), 2, "0")); the day is $(round(Int, day_frac * 100))% gone")
    println("  week $(week(t)), day $(dayofweek(t)) of 7")
    println()

    note =
        day_frac < 0.25 ? "  early water; the quiet part of the day" :
        day_frac < 0.5 ? "  rising; most of the morning is still yours" :
        day_frac < 0.75 ? "  past the meridian; spend the afternoon on purpose" :
        "  ebb; write down the one thing that mattered"
    println(note)
    println()
end

main()
