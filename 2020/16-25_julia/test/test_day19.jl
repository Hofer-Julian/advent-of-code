using Test
include("../src/day19.jl")

@testset "Day19 part1: Number of messages matching rule 0" begin
    example_1 = """
0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb"""

    rules, messages = day19.parse_input(example_1)
    @test day19.number_match_rule_zero(rules, messages) == 2
    
end