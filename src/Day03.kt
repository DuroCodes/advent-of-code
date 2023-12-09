fun main() {
    val d = Day03(readInput("input"))
    println(d.solvePart1())
    println(d.solvePart2())
}

class Day03(input: List<String>) {
    private val engine = parseEngine(input)

    fun solvePart1() = engine.sumOfParts
    fun solvePart2() = engine.sumOfGears

    private fun parseEngine(input: List<String>): Engine {
        var parts = 0
        val asterisks = mutableMapOf<Vector2D, MutableList<Int>>()

        input.forEachIndexed { y, line ->
            var x = 0
            while (x < line.length) {
                if (!line[x].isDigit()) {
                    x++
                    continue
                }

                val num = line.substring(x).takeWhile { it.isDigit() }
                val top = (x until x + num.length).map { Vector2D(it, y - 1) }
                val bottom = (x until x + num.length).map { Vector2D(it, y + 1) }
                val left = (y - 1..y + 1).map { Vector2D(x - 1, it) }
                val right = (y - 1..y + 1).map { Vector2D(x + num.length, it) }

                listOf(top, bottom, left, right).flatten().filter { it.y in input.indices && it.x in line.indices }
                    .forEach { n ->
                        if (input[n.y][n.x] != '.') {
                            parts += num.toInt()
                        }
                        if (input[n.y][n.x] == '*') {
                            asterisks.getOrPut(n) { mutableListOf() }.add(num.toInt())
                        }
                    }

                x += num.length
            }
        }

        return Engine(sumOfParts = parts, sumOfGears = asterisks.filterValues {
            it.size == 2
        }.values.sumOf {
            it.reduce { acc, i -> acc * i }
        })
    }
}

data class Engine(val sumOfParts: Int, val sumOfGears: Int)