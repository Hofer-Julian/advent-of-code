module day16

function read_file()
    lines = open("$(@__DIR__)/../input/day16.txt") do f
        input = read(f, String)
        parse_file(input)
    end
end

function parse_file(input::AbstractString)
    input = strip(input)
    rules_complete, mine_complete, nearby_complete = split(input, "\n\n")

    mine = split(mine_complete, ":\n")[2]
    nearby = split(nearby_complete, ":\n")[2]
    
    split(rules_complete, "\n"), split(mine, "\n"), split(nearby, "\n") 
end


function ticket_error_rate(rules, nearby)
    ranges = Set()
    for rule in rules
        m = match(r".*: (\d+)-(\d+) or (\d+)-(\d+)", rule)

        
        range_1 = (lower = parse(Int, m[1]), upper = parse(Int, m[2]))
        range_2 = (lower = parse(Int, m[3]), upper = parse(Int, m[4]))

        push!(ranges, range_1)
        push!(ranges, range_2)
    end

    error_rate = 0
    for fields in nearby
        fields_array = split(fields, ",")
        for field in fields_array
            field = parse(Int, field)
            for range in ranges
                if range.lower <= field <= range.upper
                    @goto field_loop
                end
            end
            error_rate += field
            @label field_loop
        end    
    end
    error_rate
end



function run()
    rules, mine, nearby = read_file()
    println("The answer to the first part is $(ticket_error_rate(rules, nearby))") 
end

end