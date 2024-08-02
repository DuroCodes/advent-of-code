fun main() {
    val d = Day01(readInput("input"))
    d.solve()
}

private class Day01(private val input: List<String>) : Day {
    private val words = mapOf(
        "one" to 1,
        "two" to 2,
        "three" to 3,
        "four" to 4,
        "five" to 5,
        "six" to 6,
        "seven" to 7,
        "eight" to 8,
        "nine" to 9,
    )

    private fun calibrateRow(row: String): Int {
        val first = row.first { it.isDigit() }
        val last = row.last { it.isDigit() }
        return "$first$last".toInt()
    }

    override fun solvePart1() = input.sumOf { calibrateRow(it) }

    override fun solvePart2() = input.sumOf { r ->
        calibrateRow(
            r.mapIndexedNotNull { i, c ->
                if (c.isDigit()) c
                else r.possibleWordsAt(i).firstNotNullOfOrNull { words[it] }
            }.joinToString("")
        )
    }
}

private fun String.possibleWordsAt(i: Int): List<String> = (3..5).map { substring(i, (i + it).coerceAtMost(length)) }

