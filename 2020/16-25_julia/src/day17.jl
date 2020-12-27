module day17

function read_file()
    lines = open("$(@__DIR__)/../input/day17.txt") do f
        input = read(f, String)
        parse_file(input)
    end
end

function parse_file(input::AbstractString)
    array_dim = 100

    lines = split(input, "\n")
    x_length = length(lines[1])
    y_length = length(lines)
    inital_x::Int = ceil(array_dim / 2 - x_length / 2)
    inital_y::Int = ceil(array_dim / 2 - y_length / 2)
    inital_z::Int = ceil(array_dim / 2)

    inital_state = falses(array_dim, array_dim, array_dim)

    for (y, line) in enumerate(lines)
        for (x, state) in enumerate(line)
            if state == '#'
                inital_state[CartesianIndex(inital_x + x, inital_y + y, inital_z)] = true
            end
        end
    end
    inital_state
end

function active_cubes_after_boot(initial_state)
    old_state = copy(initial_state)
    new_state = copy(old_state)
    for _ in 1:6
        for index in CartesianIndices(old_state)
            change_state!(new_state, old_state, index)
        end
        old_state = copy(new_state)
    end
    number_of_true_elements(new_state)
end

function number_of_true_elements(array)
    filter(e -> e, array) |> count
end

function change_state!(new_state, old_state, index)
    number_of_active_neighbors = get_number_of_active_neighbors(old_state, index)
    # @show number_of_active_neighbors
    if old_state[index] == true && !(number_of_active_neighbors in [2, 3])
        new_state[index] = false
    end

    if old_state[index] == false && number_of_active_neighbors == 3
        new_state[index] = true
    end
end

function get_number_of_active_neighbors(old_state, current_index)
    # For details see https://julialang.org/blog/2016/02/iteration/#a_multidimensional_boxcar_filter
    iter = CartesianIndices(old_state)
    ifirst, ilast = first(iter), last(iter) # in order to avoid getting out-of-bounds
    i1 = oneunit(ifirst)

    number_of_active_neighbors = 0
    for neighbor_index in max(ifirst, current_index - i1):min(ilast, current_index + i1)
        if neighbor_index == current_index
            continue
        end
        if old_state[neighbor_index] == true
            number_of_active_neighbors += 1
        end
    end
    
    number_of_active_neighbors
end

function run()
    inital_state = read_file()
    println("The answer to the first part is $(active_cubes_after_boot(inital_state))")
end

end