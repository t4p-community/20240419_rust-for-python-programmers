from functools import reduce


def main() -> None:
    # create a range of numbers from 0 to 9
    nums = range(10)

    print(nums)

    print(f"Length of nums 1: {len(nums)}")

    # initial value of acc is 0, and for each element in nums, increment acc
    # by 1 _ means we don't care about the value of the element, just the fact
    # that it exists
    len_of_nums = reduce(lambda acc, _: acc + 1, nums, 0)
    print(f"Length of nums 2: {len_of_nums}")

    print(f"Sum of nums 1: {sum(nums)}")

    # initial value of acc is the first element of the nums list, and for each
    # element in nums after the first element, add the element to num
    sum_of_nums = reduce(lambda acc, num: acc + num, nums)
    print(f"Sum of nums 2: {sum_of_nums}")


if __name__ == "__main__":
    main()
