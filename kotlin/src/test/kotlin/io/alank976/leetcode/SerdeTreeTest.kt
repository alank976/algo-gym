package io.alank976.leetcode


import Codec
import io.kotlintest.data.forall
import io.kotlintest.specs.StringSpec
import io.kotlintest.shouldBe
import io.kotlintest.tables.row

class SerdeTreeTest : StringSpec() {

    init {
        "it works" {
            forall(
//                row("1,2,X,X,3,4,X,X,5,X,X", "")
                row("1,2,3,X,X,4,5,X,X,X,X", "levelOrder")
            ) { input, _ ->
                Codec().run {
                    val root = deserialize(input)
                    serialize(root) shouldBe input
                }
            }
        }
    }

}
