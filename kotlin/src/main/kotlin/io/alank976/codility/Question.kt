package io.alank976.codility

class Question {

    fun solution(a: IntArray): Int {
        val max = a.max() ?: 0
        val set = a.toSet()
        return (1..max + 1).firstOrNull { it !in set } ?: 1
    }

    data class FoldState(var maxDistance: Int = 0, var prevIndex: Int)

    fun solution1(a: IntArray): Int {
        val adjacentIndices = a
                .withIndex()
                .sortedBy { (_, v) -> v }
                .groupBy({ (_, v) -> v }) { (i, _) -> i }
                .mapValues { (_, indicesWithSameValue) -> indicesWithSameValue.max()!! }
                .values
        val firstIndex = adjacentIndices.first()
        return adjacentIndices.fold(FoldState(prevIndex = firstIndex)) { state, i ->
            val distance = i - state.prevIndex
            if (distance > state.maxDistance) {
                state.maxDistance = distance
            }
            state.prevIndex = i
            state
        }.maxDistance
    }


    fun solution2(a: IntArray): Int {
        var i = 0
        var maxSliceLength = 0

        while (i < a.size - 1) {
            var j: Int = i + 1
            var firstSeenTheOtherValueAt: Int? = null

            while (j < a.size && (i..j).map { a[it] }.toSet().size <= 2) {
                if (firstSeenTheOtherValueAt == null && a[j] != a[i]) {
                    firstSeenTheOtherValueAt = j
                }
                j++
            }
            val sliceLength = j - i
            if (sliceLength > maxSliceLength) {
                maxSliceLength = sliceLength
            }
            i = firstSeenTheOtherValueAt ?: i + 1
            firstSeenTheOtherValueAt = null
        }
        return maxSliceLength
    }


    // Must be sorted before
    fun IntArray.isConsecutive(): Boolean = last() - first() + 1 == size


}
