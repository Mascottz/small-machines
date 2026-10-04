# small machines; night. the sky, computed not fetched. one file,
# stdlib only; it exists as itself.
#
#   julia night.jl                      tonight, from lagos
#   julia night.jl 2026-12-14           any date, 21:00 utc
#   julia night.jl 2026-12-14 6.45 3.39 date, latitude, longitude
#   julia night.jl --selftest           the pinned checks
#
# mean keplerian elements from the jpl approximate ephemeris, a
# truncated moon theory, and a small catalog of bright stars. good to
# about a degree near this epoch, and the readme of the sky says so.

using Dates

const OBLIQUITY = deg2rad(23.4392911)

function julian(dt::DateTime)
    y, m = year(dt), month(dt)
    d = day(dt) + (hour(dt) + minute(dt) / 60 + second(dt) / 3600) / 24
    if m <= 2
        y -= 1
        m += 12
    end
    a = div(y, 100)
    b = 2 - a + div(a, 4)
    return floor(Int, 365.25 * (y + 4716)) + floor(Int, 30.6001 * (m + 1)) + d + b - 1524.5
end

# a, L, e, varpi, and their rates per century; jpl approximate table
const PLANETS = [
    ("mercury", 0.38709927, 252.25032350, 0.20563593, 77.45779628, 0.00000037, 149472.67411175, 0.00001906, 0.16047689),
    ("venus", 0.72333566, 181.97909950, 0.00677672, 131.60246718, 0.00000000, 58517.81538729, -0.00004107, 0.00268329),
    ("mars", 1.52371034, -4.55343205, 0.09339410, -23.94362959, 0.00001847, 19140.30268499, 0.00007882, 0.44441088),
    ("jupiter", 5.20288700, 34.39644051, 0.04838624, 14.72847983, -0.00011607, 3034.74612775, -0.00013253, 0.21252668),
    ("saturn", 9.53667594, 49.95424423, 0.05386179, 92.59887831, -0.00125060, 1222.49362201, -0.00050991, -0.41897216),
]
const EARTH = (1.00000261, 100.46457166, 0.01671123, 102.93768193, 0.00000562, 35999.37244981, -0.00004022, 0.32327364)

const MAGS = Dict("mercury" => -0.5, "venus" => -4.4, "mars" => -1.5, "jupiter" => -2.5, "saturn" => 0.6)

const STARS = [
    ("sirius", 101.287, -16.716, -1.46),
    ("canopus", 95.988, -52.696, -0.74),
    ("alpha centauri", 219.902, -60.834, -0.27),
    ("arcturus", 213.915, 19.182, -0.05),
    ("vega", 279.234, 38.784, 0.03),
    ("capella", 79.172, 45.998, 0.08),
    ("rigel", 78.634, -8.202, 0.13),
]

function heliocentric(a, L, e, varpi, T)
    M = deg2rad(L - varpi)
    E = M
    for _ in 1:7
        E = M + e * sin(E)
    end
    nu = 2 * atan(sqrt(1 + e) * sin(E / 2), sqrt(1 - e) * cos(E / 2))
    r = a * (1 - e * cos(E))
    lam = nu + deg2rad(varpi)
    return (r * cos(lam), r * sin(lam), 0.0)
end

function equatorial(x, y, z)
    yy = y * cos(OBLIQUITY) - z * sin(OBLIQUITY)
    zz = y * sin(OBLIQUITY) + z * cos(OBLIQUITY)
    ra = rad2deg(atan(yy, x))
    r = sqrt(x^2 + yy^2 + zz^2)
    dec = rad2deg(asin(zz / r))
    return (mod(ra, 360), dec)
end

function altitude(ra, dec, jd, lat, lon)
    n = jd - 2451545.0
    gmst = mod(280.46061837 + 360.98564736629 * n, 360)
    ha = deg2rad(mod(gmst + lon - ra, 360))
    la, de = deg2rad(lat), deg2rad(dec)
    return rad2deg(asin(sin(la) * sin(de) + cos(la) * cos(de) * cos(ha)))
end

function sun_ecliptic(n)
    L = 280.460 + 0.9856474n
    g = deg2rad(357.528 + 0.9856003n)
    lam = deg2rad(L + 1.915 * sin(g) + 0.020 * sin(2g))
    dist = 1.00014 - 0.01671 * cos(g) - 0.00014 * cos(2g)
    return (dist * cos(lam), dist * sin(lam), 0.0, mod(rad2deg(lam), 360))
end

function moon_state(jd)
    d = jd - 2451545.0
    M = deg2rad(134.963 + 13.064993d)
    F = deg2rad(93.272 + 13.229350d)
    lam = deg2rad(218.316 + 13.176396d + 6.289 * sin(M))
    beta = deg2rad(5.128 * sin(F))
    x, y, z = cos(lam) * cos(beta), sin(lam) * cos(beta), sin(beta)
    (ra, dec) = equatorial(x, y, z)
    age = mod(jd - 2451550.1, 29.530588853)
    illum = (1 - cos(2π * age / 29.530588853)) / 2
    name =
        age < 1.8 ? "new" :
        age < 5.5 ? "waxing crescent" :
        age < 9.2 ? "first quarter" :
        age < 12.9 ? "waxing gibbous" :
        age < 16.6 ? "full" :
        age < 20.3 ? "waning gibbous" :
        age < 24.0 ? "last quarter" :
        age < 27.7 ? "waning crescent" : "new"
    return (ra, dec, age, illum, name)
end

function cardinal(az)
    dirs = ["north", "northeast", "east", "southeast", "south", "southwest", "west", "northwest"]
    dirs[mod(round(Int, az / 45), 8) + 1]
end

function azimuth(ra, dec, jd, lat, lon)
    n = jd - 2451545.0
    gmst = mod(280.46061837 + 360.98564736629 * n, 360)
    ha = deg2rad(mod(gmst + lon - ra, 360))
    la, de = deg2rad(lat), deg2rad(dec)
    az = rad2deg(atan(-cos(de) * sin(ha), sin(de) * cos(la) - cos(de) * cos(ha) * sin(la)))
    return mod(az, 360)
end

function fmt_alt(a)
    sign = a >= 0 ? "+" : "-"
    "$sign$(round(Int, abs(a)))°"
end

function run(dt::DateTime, lat, lon)
    jd = julian(dt)
    n = jd - 2451545.0
    T = n / 36525

    eL = EARTH[2] + EARTH[6] * T
    ee = EARTH[3] + EARTH[7] * T
    eva = EARTH[4] + EARTH[8] * T
    (ex, ey, ez) = heliocentric(EARTH[1], eL, ee, eva, T)

    println()
    println("  small machines · night")
    println("  $(Dates.format(dt, "yyyy-mm-dd HH:MM")) utc, lat $(round(lat, digits=2)), lon $(round(lon, digits=2))")
    println()

    (mra, mdec, age, illum, mname) = moon_state(jd)
    malt = altitude(mra, mdec, jd, lat, lon)
    println("  moon; $mname, $(round(Int, illum * 100))% lit, altitude $(fmt_alt(malt))")

    println("  planets above the horizon;")
    candidates = Tuple{String,Float64,Float64}[]
    if illum > 0.10 && malt > 5
        push!(candidates, ("the moon", -12.6, malt))
    end
    for (name, a, L, e, varpi, da, dL, de, dvarpi) in PLANETS
        pL = L + dL * T
        pe = e + de * T
        pva = varpi + dvarpi * T
        (px, py, pz) = heliocentric(a, pL, pe, pva, T)
        (gx, gy, gz) = (px - ex, py - ey, pz - ez)
        (ra, dec) = equatorial(gx, gy, gz)
        alt = altitude(ra, dec, jd, lat, lon)
        if alt > 0
            az = azimuth(ra, dec, jd, lat, lon)
            println("    $(rpad(name, 8)) alt $(fmt_alt(alt)), $(cardinal(az))")
            push!(candidates, (name, MAGS[name], alt))
        end
    end
    for (sname, ra, dec, mag) in STARS
        alt = altitude(ra, dec, jd, lat, lon)
        if alt > 10
            push!(candidates, (sname, mag, alt))
        end
    end

    println()
    if isempty(candidates)
        println("  the one bright thing; nothing bright is up; the dark is the point")
    else
        best = first(sort(candidates, by = c -> c[2]))
        println("  the one bright thing; $(best[1]) at $(fmt_alt(best[3])), magnitude $(best[2])")
    end
    println()
end

function selftest()
    ok = true
    ok &= abs(julian(DateTime(2000, 1, 1, 12)) - 2451545.0) < 1e-6
    n = julian(DateTime(2026, 3, 20, 9)) - 2451545.0
    (_, _, _, sunlon) = sun_ecliptic(n)
    ok &= min(sunlon, 360 - sunlon) < 1.5
    (ra, dec, age, illum, name) = moon_state(julian(DateTime(2000, 1, 6, 18)))
    ok &= age < 0.5 && name == "new"
    (_, _, fage, fillum, fname) = moon_state(julian(DateTime(2000, 1, 21, 4)))
    ok &= fillum > 0.95 && fname == "full"
    return ok
end

args = ARGS
if "--selftest" in args
    println(selftest() ? "  night; the checks hold" : "  night; a check broke")
    exit(selftest() ? 0 : 1)
end

dt = now()
lat, lon = 6.45, 3.39
if length(args) >= 1
    dt = DateTime(args[1]) + Hour(21)
end
if length(args) >= 3
    lat = parse(Float64, args[2])
    lon = parse(Float64, args[3])
end
run(dt, lat, lon)
