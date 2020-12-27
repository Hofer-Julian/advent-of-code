using Test
include("../src/day17.jl")

@testset "Day17 part1: Number of active cubes after boot in 3D" begin
    input = """
.#.
..#
###"""

    inital_state = day17.parse_file_part1(input)
    @test day17.active_cubes_after_boot(inital_state) == 112
end

@testset "Day17 part2: Number of active cubes after boot in 4D" begin
    input = """
.#.
..#
###"""

    inital_state = day17.parse_file_part2(input)
    @test day17.active_cubes_after_boot(inital_state) == 848
end