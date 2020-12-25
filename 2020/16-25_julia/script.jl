#!/usr/bin/env julia
using Pkg
Pkg.activate(@__DIR__)
Pkg.instantiate()

using ArgParse
function parse_commandline()
    s = ArgParseSettings()
    @add_arg_table s begin
        "--day"
            help = "which day to run (e.g. --day=16)"
        "--test"
            help = "wether to run the tests"
            action = :store_true
    end
    return parse_args(s)
end

parsed_args = parse_commandline()
for (arg, val) in parsed_args
    if arg == "day" && val !== nothing
        include("src/day$(val).jl")
        aoc.run()

    elseif arg == "test" && val == true
        Pkg.test()
    end
end
