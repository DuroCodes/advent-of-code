fun main() {
    val d = Day20(readInput("input"))
    d.solve()
}

private class Day20(input: List<String>) : Day {
    val parsed = input.map {
        it.split(" -> ").let { (a, b) ->
            val type = if (a.contains('%') || a.contains('&')) a[0] else null
            val rest = if (a.contains('%') || a.contains('&')) a.substring(1..<a.length) else a
            when (type) {
                null -> Start(rest, b.split(", "))
                '%' -> Flip(rest, b.split(", "))
                '&' -> And(rest, b.split(", "))
                else -> error("")
            }
        }
    }.associateBy { it.key }.also { map ->
        map.forEach { (key, value) ->
            for (o in value.output) {
                when (val target = map[o]) {
                    is And -> target.input[key] = false
                    else -> Unit
                }
            }
        }
    }

    private fun sim() = sequence {
        val nodes = mutableListOf(Triple("input", "broadcaster", false))
        while (nodes.isNotEmpty()) {
            val (from, to, signal) = nodes.removeFirst()
            yield(signal)
            val (output, sent) = parsed[to]?.receive(from, signal) ?: continue
            for (o in output) {
                nodes.add(Triple(to, o, sent))
            }
        }
    }

    override fun solvePart1() =
        generateSequence { sim() }.take(1000).flatten().fold(0 to 0) { (t, f), b -> if (b) t + 1 to f else t to f + 1 }
            .let { (t, f) -> t * f }

    override fun solvePart2(): Long {
        parsed.values.forEach(Module::reset)
        val flip = parsed.values.filterIsInstance<Flip>()
        val states = BooleanArray(flip.size) { false }
        val flips = List(flip.size) { mutableListOf<Int>() }
        var totalIdx = 0
        while (flips.any { it.size < 2 }) {
            sim().last()
            flip.forEachIndexed { i, t ->
                if (t.lastSignal != states[i]) {
                    states[i] = t.lastSignal
                    flips[i] += totalIdx + 1
                }
            }
            totalIdx++
        }

        return flips.map { it[1].toLong() }.filter { it.countOneBits() != 1 }.reduce(::lcm)
    }
}

private sealed interface Module {
    val lastSignal: Boolean
    val key: String
    val output: List<String>
    fun receive(from: String, signal: Boolean): Pair<List<String>, Boolean>
    fun reset() {}
}

private data class Start(override val key: String, override val output: List<String>) : Module {
    override val lastSignal = false
    override fun receive(from: String, signal: Boolean) = output to signal
}

private data class Flip(override val key: String, override val output: List<String>) : Module {
    override var lastSignal = false
    override fun receive(from: String, signal: Boolean): Pair<List<String>, Boolean> {
        return when (signal) {
            true -> emptyList()
            false -> output.also { lastSignal = !lastSignal }
        } to lastSignal
    }

    override fun reset() {
        lastSignal = false
    }
}

private data class And(override val key: String, override val output: List<String>) : Module {
    val input = mutableMapOf<String, Boolean>()
    override val lastSignal get() = !input.values.all { it }
    override fun receive(from: String, signal: Boolean): Pair<List<String>, Boolean> {
        input[from] = signal
        return output to lastSignal
    }

    override fun reset() {
        input.mapValues { false }
    }
}