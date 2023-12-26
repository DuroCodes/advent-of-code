fun main() {
    val d = Day25(readInput("input"))
    d.solve()
}

private class Day25(input: List<String>) : Day {
    val parsed = parse {
        val lines = input.map { it.split(": ", " ").filterNot(String::isBlank) }
        val wires = lines.flatten().toSet()
        val edges = lines.flatMap { l -> l.drop(1).map { l.first() to it } }.toSet()
        SnowMachineWiring(wires, edges)
    }

    override fun solvePart1() = parsed.solve()
    override fun solvePart2() = 0
}

private data class SnowMachineWiring(val wires: Set<String>, val edges: Set<Pair<String, String>>) {
    fun solve(): Int {
        while (true) {
            val subgroups = wires.map { mutableSetOf(it) }.toMutableSet()
            val subgroupContains = { w: String -> subgroups.firstOrNull { w in it } }
            val randomEdge = edges.shuffled().map { it.toList() }.iterator()

            while (subgroups.size > 2) {
                val (a, b) = randomEdge.next().map(subgroupContains)
                if (a == b || a == null || b == null) continue
                a.addAll(b)
                subgroups.removeIf { it == b }
            }

            if (edges.count { (a, b) -> subgroupContains(a) != subgroupContains(b) } > 3) continue

            return subgroups.map { it.size }.reduce(Int::times)
        }
    }
}