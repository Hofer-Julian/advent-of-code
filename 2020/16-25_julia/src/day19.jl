module day19


function read_file()
    lines = open("$(@__DIR__)/../input/day19.txt") do f
        read(f, String) |> chomp        
    end
end


function parse_input(input)
    rules, messages = split(input, "\n\n")
    split(rules, "\n"), split(messages, "\n")
end


function number_match_rule_zero(rules, messages)
    working_dict, finished_dict = get_starting_dicts(rules)

    while true
        changed = false
        for (index, numbers) in working_dict
            if finished_dict_contains_numbers(finished_dict, numbers)
                new_addition = []
                for first_dim in numbers
                    old_names = copy(finished_dict[first_dim[1]])
                    for (index, number) in enumerate(first_dim)
                        if index == 1
                            old_names = finished_dict[number]
                            continue
                        end
                        new_names = []
                        for old_name in old_names
                            for new_name in finished_dict[number]
                                push!(new_names, old_name * new_name)
                            end
                        end
                        old_names = new_names
                    end
                    append!(new_addition, old_names)
                end
                finished_dict[index] = new_addition
                delete!(working_dict, index)
            end
        end
        
        if isempty(working_dict)
            return filter(m -> m ∈ finished_dict["0"], messages) |> length
        end
    end
end


function finished_dict_contains_numbers(finished_dict, numbers)

    for firstdim in numbers
        for number in firstdim
            if number ∉ keys(finished_dict)
                return false
            end
        end
    end
    true
end


function get_starting_dicts(rules)
    re_general = r"(\d+): (.*)"
    re_inclusive_four = r"(\d+) (\d+) \| (\d+) (\d+)"
    re_inclusive_two = r"(\d+) \| (\d+)"    
    re_one = r"(\d+)"
    re_two = r"(\d+) (\d+)"
    re_three = r"(\d+) (\d+) (\d+)"
    re_letter = r"\"(\w)\""

    finished_dict = Dict()
    working_dict = Dict()

    
    for rule in rules
        m = match(re_general, rule)
        if m === nothing
            throw(ErrorException("This rule does not match the basic pattern: $rule"))
        end
        index = m.captures[1]
        context = m.captures[2]
        

        m = match(re_letter, context)
        if m !== nothing
            finished_dict[index] = [m.captures[1]]
            continue
        end

        m = match(re_inclusive_four, context)
        if m !== nothing
            working_dict[index] = [[m.captures[1], m.captures[2]],
                                   [m.captures[3], m.captures[4]]]
            continue
        end

        m = match(re_inclusive_two, context)
        if m !== nothing
            working_dict[index] = [[m.captures[1]],
                                   [m.captures[2]]]
            continue
        end

        m = match(re_three, context)
        if m !== nothing
            working_dict[index] = [[m.captures[1], m.captures[2], m.captures[3]]]
            continue
        end

        m = match(re_two, context)
        if m !== nothing
            working_dict[index] = [[m.captures[1], m.captures[2]]]
            continue
        end


        m = match(re_one, context)
        if m !== nothing
            working_dict[index] = [[m.captures[1]]]
            continue
        end

        throw(ErrorException("This rule could not be parsed: $rule"))

    end
    working_dict, finished_dict
end

function run()
    rules, messages = read_file() |> parse_input
    @time answer_1 = number_match_rule_zero(rules, messages)
    println("The answer to the first part is $answer_1")
end

end