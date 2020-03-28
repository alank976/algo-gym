package io.alank976.codility

import io.kotlintest.data.forall
import io.kotlintest.shouldBe
import io.kotlintest.specs.StringSpec
import io.kotlintest.tables.row

class QuestionTest : StringSpec() {

    init {
//        "demo works" {
//            forall(
//                    row(intArrayOf(1, 3, 6, 4, 1, 2), 5)
//            ) { input, expected ->
//                Question().solution(input) shouldBe expected
//            }
//        }

        "q1 works" {
            forall(
                    row(intArrayOf(1, 4, 7, 3, 3, 5), 4)
            ) { input, expected ->
                Question().solution1(input) shouldBe expected
            }
        }

        "q2 helper test" {
            forall(
                    row(intArrayOf(4,4), 2)/*,
                    row(intArrayOf(1,2,3,2), 3),
                    row(intArrayOf(0,5,4,4,5,12), 4),
                    row(intArrayOf(0,5,4,4,5,12), 4),*/
            ) { input, expected ->
                Question().solution2(input) shouldBe expected
            }
        }
    }

}
