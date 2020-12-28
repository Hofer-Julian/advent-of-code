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


function run()
    input = read_file() |> parse_input
    println("The answer to the first part is $input")
end

end