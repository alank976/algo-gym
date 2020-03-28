package io.alank976.leetcode

import io.kotlintest.data.forall
import io.kotlintest.shouldBe
import io.kotlintest.specs.StringSpec
import io.kotlintest.tables.row

class ReorganizeStringTest : StringSpec() {

    init {
        "it works" {
            forall(
                    row("aab", "aba"),
                    row("vvvlo", "vlvov"),
                    row("baaba", "ababa"),
                    row("aaab", "")
            ) { input, expected ->
                Solution().reorganizeString(input) shouldBe expected
            }
        }
    }

}
