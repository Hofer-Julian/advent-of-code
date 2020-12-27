using Test
include("../src/day16.jl")

@testset "Day16 part1: Error rate" begin
    input = """
class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12"""

    rules, mine, nearby = day16.parse_file(input)
    @test day16.ticket_error_rate(rules, nearby) == 71
end


@testset "Day16 part2: Determine fields" begin
    input = """
class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9"""

    rules, mine, nearby = day16.parse_file(input)
    ticket_fields = day16.get_my_ticket_fields(rules, mine, nearby)
    @test ticket_fields["class"] == 12
    @test ticket_fields["row"] == 11
    @test ticket_fields["seat"] == 13
end