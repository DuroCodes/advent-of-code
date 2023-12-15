fun main() {
    val d = Day15(readInput("input"))
    println(d.solvePart1())
    println(d.solvePart2())
}

private class Day15(private val input: List<String>) {
    private fun asciiTotal(s: String): Int {
        var result = 0
        s.forEach {
            result = ((result + it.code) * 17) % 256
        }
        return result
    }

    private fun focusPower(list: List<Pair<String, String>>): Long {
        val boxes = mutableMapOf<Int, LinkedHashMap<String, Int>>()

        list.forEach {
            boxes.getOrPut(asciiTotal(it.first)) { linkedMapOf() }.let { m ->
                if (it.second.isBlank()) m.remove(it.first)
                else m[it.first] = it.second.toInt()
            }
        }

        return boxes.map {
            it.value.toList().mapIndexed { i, l -> (it.key + 1) * (i + 1) * l.second }.sum().toLong()
        }.sum()
    }

    fun solvePart1() = input.first().split(",").sumOf { asciiTotal(it) }
    fun solvePart2() = input.first().split(",").map { l ->
        l.split(Regex("[\\-=]")).let {
            it.first() to it.last()
        }
    }.let { focusPower(it) }
}