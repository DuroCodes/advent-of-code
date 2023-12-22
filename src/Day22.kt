fun main() {
    val d = Day22(readInput("input"))
    d.solve()
}

private class Day22(input: List<String>) : Day {
    private val bricks = input.mapIndexed { i, l -> Brick.from(i, l) }.sortedBy { it.zRange.first }
    private val supports = mutableMapOf<Int, MutableSet<Int>>()
    private val supported = mutableMapOf<Int, MutableSet<Int>>()

    init {
        val maxes = mutableMapOf<Vector2D<Int>, Pair<Int, Int>>().withDefault { -1 to 0 }
        for (brick in bricks) {
            val points = brick.points()
            val maxZ = points.map { maxes.getValue(it) }.maxOf { it.second }
            brick.zRange = maxZ + 1 until maxZ + 1 + (brick.zRange.last - brick.zRange.first + 1)

            for (pos in points) {
                val (id, z) = maxes.getValue(pos)
                if (z == maxZ && id != -1) {
                    supports.getOrPut(id) { mutableSetOf() }.add(brick.id)
                    supported.getOrPut(brick.id) { mutableSetOf() }.add(id)
                }
                maxes[pos] = brick.id to brick.zRange.last
            }
        }
    }

    override fun solvePart1() =
        bricks.size - supported.values.filter { it.size == 1 }.map { it.toSet() }.reduce(Set<Int>::union).size

    override fun solvePart2() = bricks.sumOf { b ->
        val falling = mutableSetOf(b.id)
        var nextBricks = supports.getOrDefault(b.id, emptySet())

        while (nextBricks.isNotEmpty()) {
            nextBricks = buildSet {
                for (next in nextBricks) {
                    if ((supported.getValue(next) - falling).isEmpty()) {
                        falling += next
                        addAll(supports.getOrDefault(next, emptySet()))
                    }
                }
            }
        }

        falling.size - 1
    }

    private data class Brick(val id: Int, val xRange: IntRange, val yRange: IntRange, var zRange: IntRange) {
        fun points() = xRange.flatMap { x -> yRange.map { y -> Vector2D(x, y) } }

        companion object {
            fun from(index: Int, str: String): Brick {
                val (b1, b2) = str.split("~")
                val (x1, y1, z1) = b1.split(",").map { it.toInt() }
                val (x2, y2, z2) = b2.split(",").map { it.toInt() }
                return Brick(index, x1..x2, y1..y2, z1..z2)
            }
        }
    }
}