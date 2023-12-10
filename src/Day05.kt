import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.async
import kotlinx.coroutines.awaitAll
import kotlinx.coroutines.runBlocking
import java.util.concurrent.atomic.AtomicLong

fun main() {
    val d = Day05(readInput("input"))
    println(d.solvePart1())
    time { println(d.solvePart2()) }
}

private class Day05(private val input: List<String>) {
    private val almanac = Almanac(input.joinToString("\n"))
    fun solvePart1(): Long {
        return almanac.seeds.minOf { almanac.process(it) }
    }

    // this took 39686 ms on my machine (m1 macbook air) 💀
    fun solvePart2(): Long {
        val inverseAlmanac = Almanac(input.joinToString("\n"), true)

        val seedRanges = inverseAlmanac.seeds.chunked(2).map { (start, len) -> start until start + len }

        val coroutineCount = 24
        val location = AtomicLong(0L)

        return runBlocking {
            (0 until coroutineCount).map {
                async(Dispatchers.Default) {
                    var current: Long
                    while (true) {
                        current = location.getAndIncrement()
                        if (seedRanges.any { range ->
                                val seed = inverseAlmanac.process(current)
                                range.contains(seed) && almanac.process(seed) == current
                            }) break
                    }
                    current
                }
            }.awaitAll().min()
        }
    }
}

private data class Almanac(val seeds: List<Long>, val mappings: List<MappingGroup>) {

    companion object {
        operator fun invoke(string: String, inverse: Boolean = false): Almanac {
            val sections = string.split("\n\n")
            val seeds = mutableListOf<Long>()
            val mappings = mutableListOf<MappingGroup>()
            for (section in sections) {
                val lines = section.split("\n")
                val header = lines[0]
                if (header.startsWith("seeds:")) {
                    seeds += header.split(": ")[1].split(" ").map { it.toLong() }
                } else {
                    val mappingName = header.split(" ")[0]
                    val (sourceCategory, targetCategory) = mappingName.split("-to-")
                    val mappingList = mutableListOf<Mapping>()
                    for (i in 1 until lines.size) {
                        val line = lines[i]
                        if (line.isEmpty()) continue
                        val (dest, source, size) = line.split(" ").map { it.toLong() }
                        mappingList += Mapping(
                            if (inverse) dest else source,
                            if (inverse) source else dest,
                            size,
                        )
                    }
                    mappings += MappingGroup(
                        if (inverse) targetCategory else sourceCategory,
                        if (inverse) sourceCategory else targetCategory,
                        mappingList
                    )
                }
            }
            return Almanac(seeds, if (inverse) mappings.reversed() else mappings)
        }
    }

    fun process(value: Long): Long {
        var result = value

        for (mapping in mappings) {
            result = mapping.mappedValue(result)
        }

        return result
    }
}

private data class Mapping(val source: Long, val dest: Long, val size: Long) {
    fun mappedValue(value: Long): Long? {
        return if (value in source until source + size) dest + (value - source) else null
    }
}

private data class MappingGroup(val sourceCategory: String, val destCategory: String, val mappings: List<Mapping>) {
    fun mappedValue(value: Long): Long {
        return mappings.firstNotNullOfOrNull { it.mappedValue(value) } ?: value
    }
}
