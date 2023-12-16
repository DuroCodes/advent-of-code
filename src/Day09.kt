fun main() {
    val d = Day09(readInput("input"))
    d.solve()
}

private class Day09(input: List<String>) : Day {
    private val lines = input.map { it.split(" ").map(String::toInt) }

    private fun lineHistory(line: List<Int>) = buildList<List<Int>> {
        add(line)
        while (last().any { it != 0 }) {
            last().let { add((1 until it.size).map { i -> it[i] - it[i - 1] }) }
        }
        reverse()
    }

    override fun solvePart1() = lines.sumOf { lineHistory(it).fold(0L) { acc, l -> acc + l.last() } }
    override fun solvePart2() = lines.sumOf { lineHistory(it).fold(0L) { acc, l -> l[0] - acc } }
}