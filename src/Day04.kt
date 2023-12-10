fun main() {
    val d = Day04(readInput("input"))
    println(d.solvePart1())
    println(d.solvePart2())
}

private class Day04(input: List<String>) {
    private val cards = input.map { parseCard(it) }

    private fun parseCard(str: String) = str.split(':', limit = 2).let { (_, numbersStr) ->
        val (winning, have) = numbersStr.split('|', limit = 2).map { nums ->
            nums.split(' ').filter { !(it.isBlank() || it.isEmpty()) }.map { it.toInt() }
        }

        Card(winning, have)
    }

    fun solvePart1() = cards.sumOf { it.getWin() }

    fun solvePart2(): Int {
        val matches = cards.map { it.getMatches() }
        val numCards = mutableMapOf<Int, Int>()

        cards.indices.forEach { i ->
            val num = numCards.getOrPut(i) { 1 }
            (i + 1..i + matches[i]).forEach {
                val target = numCards.getOrPut(it) { 1 }
                numCards[it] = target + num
            }
        }

        return numCards.values.sum()
    }
}

private data class Card(val winning: List<Int>, val have: List<Int>) {
    fun getWin() = have.fold(1) { acc, value -> if (value in winning) acc * 2 else acc } / 2

    fun getMatches() = winning.count { it in have }
}