import java.util.*

/**
 * Your Codec object will be instantiated and called as such:
 * var obj = Codec()
 * var data = obj.encode(longUrl)
 * var ans = obj.decode(data)
 */
// https://leetcode.com/problems/serialize-and-deserialize-binary-tree/

class TreeNode(var `val`: Int) {
    var left: TreeNode? = null
    var right: TreeNode? = null
}

typealias Codec = /*PreOrder*/ LevelOrder

class LevelOrder {
    private val NONE = "X"
    private val DELIMITER = ','

    fun serialize(root: TreeNode?): String = root?.run {
        val queue: LinkedList<TreeNode?> = LinkedList(listOf(this))
        var sb = StringBuilder()
        while (queue.isNotEmpty()) {
            sb = queue.poll()
                ?.run {
                    queue.add(left)
                    queue.add(right)
                    sb.append(`val`).append(DELIMITER)
                }
                ?: sb.append(NONE).append(DELIMITER)
        }
        sb.toString()
            .run {
                if (last() == DELIMITER) {
                    substring(0 until length - 1)
                } else this
            }
    } ?: ""

    fun deserialize(data: String): TreeNode? {
        val iter = data.split(DELIMITER)
            .map { it.trim() }
            .filter { it.isNotBlank() }
            .map { it.takeIf { it != NONE }?.toInt() }
            .iterator()

        var root: TreeNode? = null
        val queue: LinkedList<TreeNode> = if (iter.hasNext()) {
            root = TreeNode(iter.next()!!.toInt())
            LinkedList<TreeNode>().apply { push(root) }
        } else LinkedList()

        while (iter.hasNext() && queue.isNotEmpty()) {
            val (left, right) = iter.next() to iter.next()
            queue.poll()?.let { node ->
                node.left = left?.let { TreeNode(it) }?.also { queue.add(it) }
                node.right = right?.let { TreeNode(it) }?.also { queue.add(it) }
            }
        }
        return root
    }
}

class PreOrder {
    private val NONE = "X"
    private val DELIMITER = ','

    // Encodes a URL to a shortened URL.
    fun serialize(root: TreeNode?): String {
        return StringBuilder().preOrderTransverse(root).toString().run {
            if (last() == DELIMITER) {
                substring(0 until length - 1)
            } else this
        }
    }

    private fun StringBuilder.preOrderTransverse(root: TreeNode?): StringBuilder =
        root?.run {
            append(`val`).append(DELIMITER)
                .preOrderTransverse(left)
                .preOrderTransverse(right)
        } ?: append(NONE).append(DELIMITER)


    // Decodes your encoded data to tree.
    fun deserialize(data: String): TreeNode? =
        data.split(DELIMITER)
            .map { it.trim() }
            .filter { it.isNotBlank() }
            .map { it.takeIf { it != NONE }?.toInt() }
            .iterator()
            .toNode()

    private fun Iterator<Int?>.toNode(): TreeNode? =
        takeIf { hasNext() }
            ?.next()
            ?.run {
                var node = TreeNode(this)
                node.left = toNode()
                node.right = toNode()
                node
            }


}


