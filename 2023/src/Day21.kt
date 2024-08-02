import java.util.PriorityQueue

fun main() {
    val d = Day21(readInput("input"))
    d.solve()
}

private class Day21(input: List<String>) : Day {
    private val grid = input.filter { it.isNotEmpty() }
    private val size = grid.size

    private val start = grid.withIndex().firstNotNullOf { (y, l) ->
        val x = l.indexOf('S')
        if (x >= 0) Vector2D(x, y) else null
    }

    private operator fun IntArray.get(y: Int, x: Int): Int = this[y * this@Day21.size + x]
    private operator fun IntArray.get(pos: Vector2D<Int>) = this[pos.x, pos.y]
    private operator fun IntArray.set(y: Int, x: Int, value: Int) {
        this[y * this@Day21.size + x] = value
    }

    private operator fun IntArray.set(pos: Vector2D<Int>, value: Int) {
        this[pos.x, pos.y] = value
    }

    private fun makeBlock(init: Iterable<IndexedValue<Vector2D<Int>>>): IntArray {
        val queue = PriorityQueue<IndexedValue<Vector2D<Int>>>(compareBy { it.index })
        val block = IntArray(size * size) { -1 }
        for (value in init) queue.add(value)

        while (queue.isNotEmpty()) {
            val (step, pos) = queue.remove()
            val existing = block[pos]
            if (existing in 0..step) continue
            check(existing < 0)
            block[pos] = step

            for (next in neighbors(pos)) {
                val candidate = block[next]
                if (candidate < 0) {
                    queue.add(IndexedValue(step + 1, next))
                }
            }
        }

        return block
    }

    private fun neighbors(pos: Vector2D<Int>) = buildList(4) {
        if (pos.x > 0 && grid[pos.x - 1][pos.y] != '#') add(Vector2D(pos.x - 1, pos.y))
        if (pos.y > 0 && grid[pos.x][pos.y - 1] != '#') add(Vector2D(pos.x, pos.y - 1))
        if (pos.x < this@Day21.size - 1 && grid[pos.x + 1][pos.y] != '#') add(Vector2D(pos.x + 1, pos.y))
        if (pos.y < this@Day21.size - 1 && grid[pos.x][pos.y + 1] != '#') add(Vector2D(pos.x, pos.y + 1))
    }

    override fun solvePart1() = makeBlock(listOf(IndexedValue(0, start))).count { it in 0..64 && it xor 64 and 1 == 0 }
    override fun solvePart2(): Long {
        val origin = makeBlock(listOf(IndexedValue(0, start)))
        var acc = origin.count { it in 0..26501365 && it xor 26501365 and 1 == 0 }.toLong()

        for (quad in 0 until 4) {
            val signY = quad and 1 == 0
            val signX = quad and 2 == 0
            val block = makeBlock(
                listOf(
                    IndexedValue(
                        origin[if (signY) 0 else size - 1, if (signX) 0 else size - 1] + 2,
                        Vector2D(if (signY) size - 1 else 0, if (signX) size - 1 else 0)
                    )
                )
            )
            acc += block.sumOf { s ->
                if (s !in 0..26501365) return@sumOf 0
                val remain = 26501365 - s
                if (remain % 2 == 0) {
                    (remain / size / 2 + 1).let { it.toLong() * it }
                } else {
                    ((remain / size + 1) / 2).let { it.toLong() * (it + 1) }
                }
            }
        }

        for (axis in 0 until 4) {
            val sign = axis and 1 == 0
            val src = if (sign) 0 else size - 1
            val dst = if (sign) size - 1 else 0
            val orientation = axis and 2 == 0
            var block = origin
            do {
                val lastBlock = block
                block = makeBlock((0..<size).map {
                    IndexedValue(
                        lastBlock[if (orientation) it else src, if (orientation) src else it] + 1,
                        Vector2D(if (orientation) it else dst, if (orientation) dst else it),
                    )
                })
                acc += block.count { it in 0..26501365 && it xor 26501365 and 1 == 0 }
            } while (block.any { it in 0..26501365 } && block.withIndex()
                    .any { (i, step) -> step >= 0 && step - lastBlock[i] != size })

            acc += block.sumOf { s ->
                if (s < 0) return@sumOf 0
                val remaining = 26501365 - s + size
                ((remaining + 1) / size - remaining.and(1)).coerceAtLeast(0) / 2
            }
        }

        return acc
    }
}