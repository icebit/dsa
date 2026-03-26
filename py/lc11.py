def containerWithMostWater(height):
    maxHeight = 0

    a = 0
    b = len(height) - 1

    while a < b:
        maxHeight = max(maxHeight, min(height[a], height[b]) * (b-a))

        if height[a] < height[b]:
            a += 1
        else:
            b -= 1

    return maxHeight
