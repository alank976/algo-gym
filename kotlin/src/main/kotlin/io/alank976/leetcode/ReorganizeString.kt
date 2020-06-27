package io.alank976.leetcode

//https://leetcode.com/problems/reorganize-string/description/

internal class Solution {
    fun reorganizeString(s: String): String {
        if (s.isBlank()) return ""
        val countByChar = s
                .groupBy { it }
                .mapValues { (_, v) -> v.size }
        val maxPossibleCountForOneChar = s.length / 2 + s.length % 2
        return if (countByChar.values.max() ?: 0 > maxPossibleCountForOneChar) ""
        else countByChar
                .map { (ch, count) -> count to ch }
                .sortedByDescending { (count, _) -> count }
                .flatMap { (count, ch) -> List(count) { ch } }
                .shuffleToAlternate(s.length)
                .joinToString(separator = "")
    }

    private inline fun <reified T> List<T>.shuffleToAlternate(n: Int) =
            zip(n.genEvens() + n.genOdds())
                    .sortedBy { (_, i) -> i }
                    .map { (item, _) -> item }

    private fun Int.genEvens(): List<Int> = (0 until this).filter { it % 2 == 0 }
    private fun Int.genOdds(): List<Int> = (0 until this).filter { it % 2 != 0 }
}
