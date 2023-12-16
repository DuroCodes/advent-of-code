fun main() {
    val d = Day02(readInput("input"))
    d.solve()
}

private class Day02(private val input: List<String>) : Day {
    private fun parseLine(line: String) = line.lineSequence().map(Game.Companion::parse)

    override fun solvePart1() = input.map(::parseLine).sumOf { games ->
        games.filter { g -> g.isPossibleWith(Game.Balls(12, 13, 14)) }.sumOf { it.id }
    }

    override fun solvePart2() = input.map(::parseLine).sumOf { games -> games.sumOf { it.minSet().power } }
}

private data class Game(val id: Int, val rounds: List<Balls>) {
    data class Balls(val red: Int, val green: Int, val blue: Int) {
        val power = red * blue * green
    }

    fun isPossibleWith(set: Balls) = rounds.all {
        it.red <= set.red && it.blue <= set.blue && it.green <= set.green
    }

    fun minSet() = Balls(rounds.maxOf { it.red }, rounds.maxOf { it.blue }, rounds.maxOf { it.green })

    companion object {
        fun parse(input: String) = Game(
            id = input.substringBefore(": ").substringAfter("Game ").toInt(),
            rounds = input.substringAfter(": ").split("; ").map { r ->
                r.split(", ", limit = 3).filterNot(String::isEmpty).map { it.split(" ", limit = 2) }
                    .associate { (count, color) -> color to count.toInt() }.run {
                        Balls(
                            red = getOrDefault("red", 0),
                            green = getOrDefault("green", 0),
                            blue = getOrDefault("blue", 0),
                        )
                    }
            },
        )
    }
}