fun main() {
    val d = Day12(readInput("input"))
    d.solve()
}

private class Day12(private val input: List<String>) : Day {
    private val cache = mutableMapOf<Pair<String, List<Int>>, Long>()

    override fun solvePart1() = input.sumOf {
        it.split(" ").let { s -> countLine(s.first(), s[1].split(",").map(String::toInt)) }
    }

    override fun solvePart2() = input.sumOf {
        it.split(" ").let { s ->
            countLine(
                "${s.first()}?".repeat(5).dropLast(1),
                "${s[1]},".repeat(5).split(",").filter { it.isNotBlank() }.map(String::toInt)
            )
        }
    }

    fun countLine(str: String, groups: List<Int>): Long {
        if (groups.isEmpty()) return if ("#" in str) 0 else 1
        if (str.isEmpty()) return 0

        return cache.getOrPut(str to groups) {
            var res = 0L
            if (str.first() in ".?") res += countLine(str.drop(1), groups)
            if (str.first() in "#?" && groups.first() <= str.length && "." !in str.take(groups.first()) && (groups.first() == str.length || str[groups.first()] != '#')) res += countLine(
                str.drop(groups.first() + 1), groups.drop(1)
            )

            res
        }
    }
}