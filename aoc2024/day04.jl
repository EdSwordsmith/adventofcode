
after(letter) = after(Val(letter))
after(::Val{'X'}) = 'M'
after(::Val{'M'}) = 'A'
after(::Val{'A'}) = 'S'

function part1(input)
    height = length(input)
    width = length(input[1])

    queue = []
    for row ∈ eachindex(input), col ∈ eachindex(input[row])
        if input[row][col] == 'X'
            push!(queue, ('X', row, col, -1:1, -1:1))
        end
    end

    count = 0

    while !isempty(queue)
        letter, row, col, r_range, c_range = popfirst!(queue)

        if letter == 'S'
            count += 1
        else
            next_letter = after(letter)
            for r ∈ r_range, c ∈ c_range
                if (r, c) != (0, 0) && (row + r) >= 1 && (row + r) <= height && (col + c) >= 1 && (col + c) <= width
                    if input[row+r][col+c] == next_letter
                        push!(queue, (next_letter, row + r, col + c, r:r, c:c))
                    end
                end
            end
        end
    end

    count
end

function part2(input)
    height = length(input)
    width = length(input[1])
    xs = 0
    for row ∈ eachindex(input), col ∈ eachindex(input[row])
        if input[row][col] == 'A' && row > 1 && row < height && col > 1 && col < width
            if ((input[row-1][col-1] == 'M' && input[row+1][col+1] == 'S') ||
                (input[row-1][col-1] == 'S' && input[row+1][col+1] == 'M')) &&
               ((input[row+1][col-1] == 'M' && input[row-1][col+1] == 'S') ||
                (input[row+1][col-1] == 'S' && input[row-1][col+1] == 'M'))
                xs += 1
            end
        end
    end
    xs
end


input = readlines()
# input = [
#     "MMMSXXMASM"
#     "MSAMXMSMSA"
#     "AMXSXMAAMM"
#     "MSAMASMSMX"
#     "XMASAMXAMM"
#     "XXAMMXXAMA"
#     "SMSMSASXSS"
#     "SAXAMASAAA"
#     "MAMMMXMMMM"
#     "MXMXAXMASX"
# ]
println("Part 1: ", part1(input))
println("Part 2: ", part2(input))
