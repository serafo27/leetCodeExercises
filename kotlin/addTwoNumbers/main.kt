data class ListNode(var value: Int, var next: ListNode? = null)

class Solution2 {
  fun addTwoNumbers(l1: ListNode?, l2: ListNode?): ListNode? {
    var a = l1
    var b = l2
    
    var result = ListNode(0)
    var current: ListNode? = result
    while (a != null && b != null && current != null) {

      val sumResult = sum(a.value, b.value)

      a = a.next
      b = b.next

      current.value += sumResult.digit
      if(a != null)
        current.next = ListNode(sumResult.remainder)

      current = current.next
    }

    return result
  }

  data class SumResult(val digit: Int, val remainder: Int)

  private fun sum(a: Int, b: Int): SumResult {
    val sum = a+b
    if (sum >= 10)
      return SumResult(sum - 10, 1)
    else 
      return SumResult(sum, 0)
  }
}

fun main() {
  // l1 = [2,4,3], l2 = [5,6,4]
  // output = [7, 0, 8]
  val a = ListNode(2)
  val b = ListNode(4)
  val c = ListNode(3)
  a.next = b
  b.next = c
  
  val d = ListNode(5)
  val e = ListNode(6)
  val f = ListNode(4)
  d.next = e
  e.next = f

  val s1 = ListNode(7)
  val s2 = ListNode(0)
  val s3 = ListNode(8)
  s1.next = s2
  s2.next = s3

  val solution = Solution2().addTwoNumbers(a, d)
  println(solution == s1)
}