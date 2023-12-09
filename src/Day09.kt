fun main() {
    val d = Day09(readInput("input"))
    println(d.solvePart1())
    println(d.solvePart2())
}

class Day09(input: List<String>) {
    private val lines = input.map { it.split(" ").map(String::toInt) }

    private fun lineHistory(line: List<Int>) = buildList<List<Int>> {
        add(line)
        while (last().any { it != 0 }) {
            last().let { add((1 until it.size).map { i -> it[i] - it[i - 1] }) }
        }
        reverse()
    }

    fun solvePart1() = lines.sumOf { lineHistory(it).fold(0L) { acc, l -> acc + l.last() } }
    fun solvePart2() = lines.sumOf { lineHistory(it).fold(0L) { acc, l -> l[0] - acc } }
}