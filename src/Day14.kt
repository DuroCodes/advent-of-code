fun main() {
    val d = Day14(readInput("input"))
    println(d.solvePart1())
    println(d.solvePart2())
}

private const val EMPTY = 0
private const val ROCK = 1
private const val CUBE = 2

private class Day14(input: List<String>) {
    private val grid: List<IntArray> = input.map { line ->
        IntArray(line.length) { i ->
            when (line[i]) {
                '.' -> EMPTY
                'O' -> ROCK
                '#' -> CUBE
                else -> error("Unexpected token ${line[i]}")
            }
        }
    }

    private fun rollNorth() {
        for (max in grid.size downTo 1) for (i in 1 until max) for (j in grid[0].indices) {
            if (grid[i][j] == ROCK && grid[i - 1][j] == EMPTY) {
                grid[i][j] = EMPTY
                grid[i - 1][j] = ROCK
            }
        }
    }

    private fun rollSouth() {
        for (min in 0..grid.size - 2) for (i in grid.size - 2 downTo min) for (j in grid[0].indices) {
            if (grid[i][j] == ROCK && grid[i + 1][j] == EMPTY) {
                grid[i][j] = EMPTY
                grid[i + 1][j] = ROCK
            }
        }
    }

    private fun rollEast() {
        for (min in 0..grid[0].size) for (j in grid[0].size - 2 downTo min) for (i in grid.indices) {
            if (grid[i][j] == ROCK && grid[i][j + 1] == EMPTY) {
                grid[i][j] = EMPTY
                grid[i][j + 1] = ROCK
            }
        }
    }

    private fun rollWest() {
        for (max in grid[0].size downTo 1) for (j in 1 until max) for (i in grid.indices) {
            if (grid[i][j] == ROCK && grid[i][j - 1] == EMPTY) {
                grid[i][j] = EMPTY
                grid[i][j - 1] = ROCK
            }
        }
    }

    private fun getLoad(grid: List<IntArray>) =
        grid.indices.sumOf { x -> (grid.size - x) * grid[x].count { n -> n == ROCK } }

    fun solvePart1(): Int {
        rollNorth()
        return getLoad(grid)
    }

    fun solvePart2(): Int {
        rollWest()
        rollSouth()
        rollEast()
        val oldGrids = mutableListOf(grid.map(IntArray::copyOf))
        for (cycle in 1 until 999999999) {
            rollNorth()
            rollWest()
            rollSouth()
            rollEast()
            val i = oldGrids.indexOfFirst { oldGrid -> grid.indices.all { i -> grid[i].contentEquals(oldGrid[i]) } }
            if (i == -1) {
                oldGrids.add(grid.map(IntArray::copyOf))
                continue
            }
            val cycleLength = cycle - i
            val remaining = 999999999 - cycle
            val offset = remaining % cycleLength
            return getLoad(oldGrids[i + offset])
        }
        return getLoad(grid)
    }
}