using Test
include("../src/day17.jl")

@testset "Number of active cubes after boot" begin
    input = """
.#.
..#
###"""

    inital_state = day17.parse_file(input)
    @test day17.active_cubes_after_boot(inital_state) == 112
end