using Test
include("../src/day18.jl")

@testset "Sum of resulting values" begin
    example_1 = "1 + 2 * 3 + 4 * 5 + 6"
    @test day18.evaluate_expression(example_1) == 71

    example_2 = "1 + (2 * 3) + (4 * (5 + 6))"
    @test day18.evaluate_expression(example_2) == 51

    example_3 = "2 * 3 + (4 * 5)"
    @test day18.evaluate_expression(example_3) == 26

    example_4 = "5 + (8 * 3 + 9 + 3 * 4 * 3)"
    @test day18.evaluate_expression(example_4) == 437

    example_5 = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"
    @test day18.evaluate_expression(example_5) == 12240

    example_6 = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"
    @test day18.evaluate_expression(example_6) == 13632

end
