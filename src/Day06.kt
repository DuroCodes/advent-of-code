fun main() {
    val d = Day06(readInput("input"))
    println(d.solvePart1())
    println(d.solvePart2())
}

class Day06(private val input: List<String>) {
    fun solvePart1() = solve(input)

    fun solvePart2() = solve(input.map { it.replace(" ", "") })

    private fun solve(input: List<String>): Long {
        val (times, distances) = input.map {
            it.substringAfter(":").split(' ').filter { s -> s.isNotBlank() }.map { s -> s.toLong() }
        }

        return times.zip(distances)
            .map { (time, distance) -> (0..time).map { it * (time - it) }.count { it > distance }.toLong() }
            .reduce(Long::times)
    }
}
