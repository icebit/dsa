def maximumSubarray(nums):
    res = nums[0]
    maxEnding = res

    for num in nums[1:]:
        if maxEnding < 0:
            maxEnding = num
        else:
            maxEnding = max(maxEnding + num, num)

        res = max(res, maxEnding)

    return res
