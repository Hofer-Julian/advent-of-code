using Test
include("../src/day18.jl")

@testset "Day18 part1: Sum of expression (left to right)" begin
    example_1 = "1 + 2 * 3 + 4 * 5 + 6"
    @test day18.evaluate_expression(example_1, day18.One) == 71
    
    example_2 = "1 + (2 * 3) + (4 * (5 + 6))"
    @test day18.evaluate_expression(example_2, day18.One) == 51

    example_3 = "2 * 3 + (4 * 5)"
    @test day18.evaluate_expression(example_3, day18.One) == 26
    
    example_4 = "5 + (8 * 3 + 9 + 3 * 4 * 3)"
    @test day18.evaluate_expression(example_4, day18.One) == 437
    
    example_5 = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"
    @test day18.evaluate_expression(example_5, day18.One) == 12240
    
    example_6 = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"
    @test day18.evaluate_expression(example_6, day18.One) == 13632
end


@testset "Day18 part1: Sum of expression (add before mul)" begin
    example_1 = "1 + 2 * 3 + 4 * 5 + 6"
    @test day18.evaluate_expression(example_1, day18.Two) == 231

    example_2 = "1 + (2 * 3) + (4 * (5 + 6))"
    @test day18.evaluate_expression(example_2, day18.Two) == 51

    example_3 = "2 * 3 + (4 * 5)"
    @test day18.evaluate_expression(example_3, day18.Two) == 46
    
    example_4 = "5 + (8 * 3 + 9 + 3 * 4 * 3)"
    @test day18.evaluate_expression(example_4, day18.Two) == 1445
    
    example_5 = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"
    @test day18.evaluate_expression(example_5, day18.Two) == 669060
    
    example_6 = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"
    @test day18.evaluate_expression(example_6, day18.Two) == 23340

end