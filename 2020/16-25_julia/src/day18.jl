module day18

function read_file()::String
    lines = open("$(@__DIR__)/../input/day18.txt") do f
        input = read(f, String) 
    end
end

function evaluate_input(input::AbstractString)::Int    
    lines = split(input, "\n")
    results = similar(lines, Int)
    @. results = evaluate_expression(lines)
    sum(results)
end

function evaluate_expression(expression::AbstractString)::Int
    1
end


function run()
    input = read_file()
    println("The answer to the first part is $(evaluate_input(input))")
end

end