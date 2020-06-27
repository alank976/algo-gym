package io.alank976.leetcode

//typealias Solution = CloneGraph


class CloneGraph {
    fun cloneGraph(node: Node?): Node? {
        val seenNodes: MutableMap<Int, Node> = mutableMapOf()
        return node?.clone(seenNodes)
    }

    private fun Node.clone(seen: MutableMap<Int, Node>): Node {
        val stored = seen[`val`]
        return stored ?: Node(`val`).also { cloned ->
            seen[`val`] = cloned
            cloned.neighbors = neighbors.map { it!!.clone(seen) }.toArrayList()
        }
    }

    private fun List<Node>.toArrayList() = ArrayList(this)
}


/**
 * Definition for a Node.

 */

class Node(var `val`: Int) {
    var neighbors: ArrayList<Node?> = ArrayList<Node?>()
}

