
class Solution1 {
  fun twoSum(nums: IntArray, target: Int): IntArray {
    nums.forEachIndexed { i, a ->
      nums.forEachIndexed { j, b -> 
        if(i != j && a + b == target)
          return intArrayOf(i, j)
      }
    }
     return intArrayOf()
  }
}

class BetterSolution {
  fun twoSum(nums: IntArray, target: Int): IntArray {
      val map = mutableMapOf<Int, Int>()
      for (i in nums.indices) {
          val complement = target - nums[i]
          if (map.containsKey(complement)) {
              return intArrayOf(map[complement]!!, i)
          }
          map[nums[i]] = i
      }
      return intArrayOf()
  }
}


fun main() {
  Solution1().twoSum(intArrayOf(3,3), 6).forEach { print(it) }
}