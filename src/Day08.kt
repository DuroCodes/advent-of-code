fun main() {
    val d = Day08(readInput("input"))
    d.solve()
}

private class Day08(input: List<String>) : Day {
    private val instructions = input.first()
    private val network = input.drop(2).associate { l ->
        l.split(" = ").let {
            val (node, targets) = it.first() to it.last().drop(1).dropLast(1).split(", ")
            node to mapOf('L' to targets.first(), 'R' to targets.last())
        }
    }

    private fun solveForNode(str: String): Long {
        var cur = str
        var i = 0
        while (!cur.endsWith('Z')) {
            val inst = instructions[i++ % instructions.length]
            cur = network[cur]!![inst]!!
        }
        return i.toLong()
    }

    override fun solvePart1() = solveForNode(network.keys.find { it == "AAA" }!!)
    override fun solvePart2() = network.keys.filter { it.last() == 'A' }.map { solveForNode(it) }.reduce(::lcm)
}