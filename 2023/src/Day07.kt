fun main() {
    val d = Day07(readInput("input"))
    d.solve()
}

private class Day07(private val input: List<String>) : Day {
    override fun solvePart1() = scoreHand(false)
    override fun solvePart2() = scoreHand(true)

    private fun cardToInt(card: Char, joker: Boolean) = when (card) {
        'A' -> 12
        'K' -> 11
        'Q' -> 10
        'J' -> if (joker) 0 else 9
        'T' -> if (joker) 9 else 8
        else -> card - (if (joker) '1' else '2')
    }

    private fun scoreHand(joker: Boolean): Int {
        val hands = input.map {
            val (cards, points) = it.split(" ")
            Hand(getRank(cards, joker), cards, points.toInt())
        }

        val comp = Comparator<Hand> { (rank1, cards1), (rank2, cards2) ->
            if (rank1 > rank2) return@Comparator 1
            if (rank1 < rank2) return@Comparator -1

            for (i in cards1.indices) {
                if (cards1[i] == cards2[i]) continue
                return@Comparator cardToInt(cards1[i], joker) - cardToInt(cards2[i], joker)
            }

            error("cards are equal somehow (this shouldn't happen)")
        }

        return hands.sortedWith(comp).mapIndexed { i, hand -> i.inc() * hand.points }.sum()
    }

    private fun getRank(cards: String, joker: Boolean): Int {
        val counts = cards.groupingBy { it }.eachCount()
        val jokers = counts['J'] ?: 0
        val newCounts = buildList<Int> {
            addAll(counts.values)
            sortDescending()
            if (joker && jokers != 5) {
                remove(jokers)
                this[0] += jokers
            }
        }

        return when {
            newCounts[0] == 5 -> 6
            newCounts[0] == 4 -> 5
            newCounts[0] == 3 && newCounts[1] == 2 -> 4
            newCounts[0] == 3 -> 3
            newCounts[0] == 2 && newCounts[1] == 2 -> 2
            newCounts[0] == 2 -> 1
            else -> 0
        }
    }
}

private data class Hand(
    val rank: Int,
    val cards: String,
    val points: Int,
)