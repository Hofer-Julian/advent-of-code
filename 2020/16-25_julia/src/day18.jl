module day18

@enum Part One Two

function read_file()::String
    lines = open("$(@__DIR__)/../input/day18.txt") do f
        input = read(f, String) 
    end
end


function evaluate_input(input::AbstractString, part::Part)::Int    
    lines = split(input, "\n")
    results = similar(lines, Int)
    @. results = evaluate_expression(lines, part)
    sum(results)
end


function evaluate_expression(expression::AbstractString, part::Part)::Int
    current_evaluation = expression
    re_single_evaluation = r"\(([^\(\)]*)\)"
    while true
        m = match(re_single_evaluation, current_evaluation)
        if m === nothing
            break
        end
        evaluated_bracket = evaluate_simple_expression(m.captures[1], part)
        current_evaluation = replace(current_evaluation, re_single_evaluation => evaluated_bracket, count=1)
    end

    evaluate_simple_expression(current_evaluation, part)
end

function evaluate_simple_expression(expression::AbstractString, part::Part)::Int
    if part == One    
        re_add = r"^(\d+) \+ (\d+)"
        re_mul = r"^(\d+) \* (\d+)"
    elseif part == Two
        re_add = r"(\d+) \+ (\d+)"
        re_mul = r"(\d+) \* (\d+)"
    end
    current_evaluation = expression
    if expression == ""
        return 0
    end
    while true
        m_add = match(re_add, current_evaluation)
        m_mul = match(re_mul, current_evaluation)
        if m_add === nothing && m_mul === nothing
            break
        end
        if m_add !== nothing
            result = parse(Int, m_add.captures[1]) + parse(Int, m_add.captures[2])
            current_evaluation = replace(current_evaluation, re_add => result, count=1)
            continue
        end

        if m_mul !== nothing
            result = parse(Int, m_mul.captures[1]) * parse(Int, m_mul.captures[2])
            current_evaluation = replace(current_evaluation, re_mul => result, count=1)
        end
    end
    
    parse(Int, current_evaluation)
end

function run()
    input = read_file()
    println("The answer to the first part is $(evaluate_input(input, One))")
    println("The answer to the second part is $(evaluate_input(input, Two))")
end

end