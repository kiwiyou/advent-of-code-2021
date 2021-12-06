File.open("input.txt") do |input|
    previous = nil
    count = 0
    input.each_line do |line|
        depth = line.to_i32
        if !previous.nil? && previous < depth
            count += 1
        end
        previous = depth
    end
    puts "#{count}"
end

File.open("input.txt") do |input|
    window = Deque(Int32).new
    sum = 0
    count = 0
    input.each_line do |line|
        depth = line.to_i32
        if window.size == 3
            new_sum = sum - window.shift + depth
            if sum < new_sum
                count += 1
            end
            sum = new_sum
        end
        window.push depth
    end
    puts "#{count}"
end