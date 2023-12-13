import kotlin.math.absoluteValue

fun main() {
    val d = Day13(readInput("input"))
    println(d.solvePart1())
    println(d.solvePart2())
}

private class Day13(input: List<String>) {
    private val patterns = parseInput(input)

    fun solvePart1() = patterns.sumOf { findMirror(it, 0) }

    fun solvePart2() = patterns.sumOf { findMirror(it, 1) }

    private fun findMirror(pattern: List<String>, goalToInt: Int) =
        findHorizontalMirror(pattern, goalToInt) ?: findVerticalMirror(pattern, goalToInt)
        ?: error("No mirror found for $pattern")

    private fun findHorizontalMirror(pattern: List<String>, goalTotal: Int) =
        (0 until pattern.lastIndex).firstNotNullOfOrNull { start ->
            if (createMirrorRanges(start, pattern.lastIndex).sumOf { (up, down) ->
                    diff(pattern[up], pattern[down])
                } == goalTotal) (start + 1) * 100
            else null
        }

    private fun findVerticalMirror(pattern: List<String>, goalTotal: Int) =
        (0 until pattern.first().lastIndex).firstNotNullOfOrNull { start ->
            if (createMirrorRanges(start, pattern.first().lastIndex).sumOf { (left, right) ->
                    diff(columnToString(pattern, left), columnToString(pattern, right))
                } == goalTotal) start + 1
            else null
        }

    fun diff(a: String, b: String) = a.indices.count { a[it] != b[it] } + (a.length - b.length).absoluteValue

    private fun createMirrorRanges(start: Int, max: Int) = (start downTo 0).zip(start + 1..max)

    private fun columnToString(list: List<String>, column: Int) = list.map { it[column] }.joinToString("")

    private fun parseInput(input: List<String>) = input.joinToString("\n").split("\n\n").map { it.lines() }
}