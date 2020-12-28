module day19

function read_file()::Array{String,1}
    lines = open("$(@__DIR__)/../input/day19.txt") do f
        readlines(f)
    end
end


function run()
    input = read_file()
    println("The answer to the first part is")
end

end