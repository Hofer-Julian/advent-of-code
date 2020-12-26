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
    
    split(rules_complete, "\n"), mine, split(nearby, "\n") 
end


function ticket_error_rate(rules, nearby)
    ranges = get_ranges(rules, nearby)

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

function get_ranges(rules, nearby)
    ranges = Set()
    for rule in rules
        m = match(r".*: (\d+)-(\d+) or (\d+)-(\d+)", rule)

        
        range_1 = (lower = parse(Int, m[1]), upper = parse(Int, m[2]))
        range_2 = (lower = parse(Int, m[3]), upper = parse(Int, m[4]))

        push!(ranges, range_1)
        push!(ranges, range_2)
    end
    ranges
end

function get_valid_nearby(rules, nearby)

    ranges = get_ranges(rules, nearby)

    valid_nearby = []
    for ticket in nearby
        fields_array = parse.(Int, split(ticket, ","))
        for field in fields_array
            for range in ranges
                if range.lower <= field <= range.upper
                    @goto field_loop
                end
            end
            @goto next_ticket # No range matched our field
            @label field_loop
        end
        push!(valid_nearby, fields_array)
        @label next_ticket
    end

    # Transorm array of array to 2D array
    hcat(valid_nearby...)
end

function get_dict_ranges(rules)
    ranges = Dict()
    for rule in rules
        m = match(r"(.*): (\d+)-(\d+) or (\d+)-(\d+)", rule)
        field = m[1]
        range_1 = (lower = parse(Int, m[2]), upper = parse(Int, m[3]))
        range_2 = (lower = parse(Int, m[4]), upper = parse(Int, m[5]))

        ranges[field] = (range_1, range_2)
    end
    ranges
end

function get_ticket_fields_column(rules, nearby)
    ticket_fields_column = Dict()
    valid_nearby = get_valid_nearby(rules, nearby)
    # Index first column with valid_nearby[1, :]
    # Index first row with valid_nearby[:, 1]

    ranges = get_dict_ranges(rules)
    while true 
        for col in 1:length(valid_nearby[:, 1]) 
            for (field, range) in ranges
                for value in valid_nearby[col, :]
                    if !(range[1].lower <= value <= range[1].upper) &&
                    !(range[2].lower <= value <= range[2].upper)
                        @goto field_loop
                    end
                end
                if only_valid_field(col, field, ticket_fields_column, ranges, valid_nearby)
                    ticket_fields_column[col] = field

                    if length(ticket_fields_column) == length(valid_nearby[:, 1])
                        return ticket_fields_column
                    end
                end
                @label field_loop
            end
        end
    end
    
end

function only_valid_field(checked_col, checked_field, ticket_fields_column, ranges,  valid_nearby)
    if checked_col in keys(ticket_fields_column)
        return false
    end
    for col in 1:length(valid_nearby[:, 1])
        if col in keys(ticket_fields_column)
            continue
        end
        if col == checked_col
            continue
        end
        for value in valid_nearby[col, :]
            if !(ranges[checked_field][1].lower <= value <= ranges[checked_field][1].upper) &&
                    !(ranges[checked_field][2].lower <= value <= ranges[checked_field][2].upper)
                @goto field_loop
            end
        end
        return false
        @label field_loop
    end
    return true
end

function get_my_ticket_fields(rules, mine,  nearby)
    ticket_fields_column = get_ticket_fields_column(rules, nearby)
    my_ticket_fields = Dict()
    my_fields = parse.(Int, split(mine, ","))
    for (col, value) in enumerate(my_fields)
        field = ticket_fields_column[col]
        my_ticket_fields[field] = value
    end
    my_ticket_fields
end

function get_answer_part_two(rules, mine, nearby)
    my_ticket_fields = get_my_ticket_fields(rules, mine, nearby)
    answer = 1
    for (field, value) in my_ticket_fields
        if startswith(field, "departure")
            answer *= value
        end
    end
    answer
end


function run()
    rules, mine, nearby = read_file()
    println("The answer to the first part is $(ticket_error_rate(rules, nearby))")
    println("The answer to the second part is $(get_answer_part_two(rules, mine, nearby))") 

end

end