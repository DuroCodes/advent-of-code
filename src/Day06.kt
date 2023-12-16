fun main() {
    val d = Day06(readInput("input"))
    d.solve()
}

private class Day06(private val input: List<String>) : Day {
    override fun solvePart1() = solve(input)

    override fun solvePart2() = solve(input.map { it.replace(" ", "") })

    private fun solve(input: List<String>): Long {
        val (times, distances) = input.map {
            it.substringAfter(":").split(' ').filter { s -> s.isNotBlank() }.map { s -> s.toLong() }
        }

        return times.zip(distances)
            .map { (time, distance) -> (0..time).map { it * (time - it) }.count { it > distance }.toLong() }
            .reduce(Long::times)
    }
}
