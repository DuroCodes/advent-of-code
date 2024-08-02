import kotlin.math.abs

fun main() {
    val d = Day11(readInput("input"))
    d.solve()
}

private class Day11(input: List<String>) : Day {
    val grid = parseGridFromString(input.joinToString("\n")) { it }

    override fun solvePart1() = process(grid, 2)
    override fun solvePart2() = process(grid, 1000000)

    fun process(grid: Map<Vector2D<Int>, Char>, multiplier: Int): Long {
        val emptyCols = (0..grid.keys.maxOf { it.x }).filter { x ->
            grid.count { it.key.x == x && it.value == '#' } == 0
        }.toSortedSet()

        val emptyRows = (0..grid.keys.maxOf { it.y }).filter { y ->
            grid.count { it.key.y == y && it.value == '#' } == 0
        }.toSortedSet()

        val galaxies = grid.filter { it.value == '#' }.keys.toList()
        var sum = 0L

        galaxies.forEachPair(true) { start, end ->
            val newStart = Vector2D(
                start.x + emptyCols.headSet(start.x).size * (multiplier - 1),
                start.y + emptyRows.headSet(start.y).size * (multiplier - 1),
            )

            val newEnd = Vector2D(
                end.x + emptyCols.headSet(end.x).size * (multiplier - 1),
                end.y + emptyRows.headSet(end.y).size * (multiplier - 1),
            )

            val manhattanDistance = abs(newStart.x - newEnd.x) + abs(newStart.y - newEnd.y)
            sum += manhattanDistance.toLong()
        }

        return sum
    }

    fun <T> parseGridFromString(string: String, converter: (Char) -> T?): Map<Vector2D<Int>, T> {
        var y = 0
        val result = mutableMapOf<Vector2D<Int>, T>()
        string.split("\n").forEach {
            var x = 0
            for (c in it) {
                converter(c)?.let {
                    result[Vector2D(x, y)] = it
                }
                x += 1
            }
            y += 1
        }
        return result
    }
}