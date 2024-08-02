import kotlin.math.abs

fun main() {
    val d = Day18(readInput("input"))
    d.solve()
}

private class Day18(input: List<String>) : Day {
    private val lines = input.map {
        val split = it.split(' ')
        Triple(split[0][0], split[1].toLong(), split[2].removeSurrounding("(#", ")"))
    }

    private val digPlan = lines.map { shiftDir(it.first) * it.second }
    private val colorsDigPlan = lines.map { shiftDir(it.third.last()) * it.third.dropLast(1).toLong(16) }

    private fun shiftDir(dir: Char) = when (dir) {
        '0', 'R' -> Vector2D(1, 0)
        '1', 'D' -> Vector2D(0, 1)
        '2', 'L' -> Vector2D(-1, 0)
        '3', 'U' -> Vector2D(0, -1)
        else -> error("Unknown direction: $dir")
    }

    private fun cornerPositions(plan: List<Vector2D<Long>>) =
        plan.scan(Vector2D(0L, 0L)) { acc, vector -> acc + vector }

    private fun perimeter(plan: List<Vector2D<Long>>) = plan.sumOf { abs(it.x) + abs(it.y) }

    private fun shoelaceFormula(corners: List<Vector2D<Long>>) = corners.zipWithNext { a, b ->
        a.x * b.y - b.x * a.y
    }.sum() / 2

    private fun pickTheorem(area: Long, perimeter: Long) = area + perimeter / 2L + 1L

    private fun lavaCapacity(plan: List<Vector2D<Long>>) =
        pickTheorem(shoelaceFormula(cornerPositions(plan)), perimeter(plan))

    override fun solvePart1() = lavaCapacity(digPlan)
    override fun solvePart2() = lavaCapacity(colorsDigPlan)
}