import java.util.*
import kotlin.io.path.Path
import kotlin.io.path.readLines

fun readInput(name: String) = Path("src/$name.txt").readLines()

interface Day {
    fun solvePart1(): Any
    fun solvePart2(): Any
    fun solve() {
        time {
            println(solvePart1())
            println(solvePart2())
        }
    }
}

data class Region2D(val xRange: IntRange, val yRange: IntRange) : Iterable<Vector2D<Int>> {
    constructor(points: Iterable<Vector2D<Int>>) : this(
        points.minOf { it.x }..points.maxOf { it.x },
        points.minOf { it.y }..points.maxOf { it.y },
    )

    fun contains(point: Vector2D<Int>) = point.x in xRange && point.y in yRange

    override fun iterator() = iterator {
        for (x in xRange) {
            for (y in yRange) {
                yield(Vector2D(x, y))
            }
        }
    }
}

inline fun <T : Any> parse(parser: () -> T): T = runCatching { parser() }.getOrElse { error(it) }

fun <T> List<T>.forEachPair(unique: Boolean = false, block: (T, T) -> Unit) {
    for (i in indices) {
        val startIndex = if (unique) i + 1 else 0
        for (j in startIndex until this.size) {
            if (i == j) continue
            block.invoke(this[i], this[j])
        }
    }
}

data class Vector2D<T : Number>(val x: T, val y: T) {
    fun move(dir: Direction) = when (dir) {
        Direction.Left -> Vector2D(x.toInt() - 1, y.toInt())
        Direction.Right -> Vector2D(x.toInt() + 1, y.toInt())
        Direction.Up -> Vector2D(x.toInt(), y.toInt() + 1)
        Direction.Down -> Vector2D(x.toInt(), y.toInt() - 1)
    }

    @JvmName("plusInt")
    operator fun plus(other: Vector2D<Int>) = Vector2D(x.toInt() + other.x, y.toInt() + other.y)

    @JvmName("plusLong")
    operator fun plus(other: Vector2D<Long>) = Vector2D(x.toLong() + other.x, y.toLong() + other.y)
    operator fun times(multiplier: Long) = Vector2D(x.toLong() * multiplier, y.toLong() * multiplier)
}

data class Vector3D(val x: Long, val y: Long, val z: Long) {
    constructor(x: Int, y: Int, z: Int) : this(x.toLong(), y.toLong(), z.toLong())
}

fun <T : Any> Iterable<T>.distinctPairs(): Sequence<Pair<T, T>> = sequence {
    val iter = this@distinctPairs.iterator()
    if (!iter.hasNext()) return@sequence
    val previous = mutableListOf(iter.next())
    while (iter.hasNext()) {
        val second = iter.next()
        for (first in previous) yield(first to second)
        previous.add(second)
    }
}

sealed interface Direction {
    sealed interface Horizontal : Direction

    sealed interface Vertical : Direction

    data object Up : Vertical

    data object Down : Vertical

    data object Left : Horizontal

    data object Right : Horizontal
}

val Direction.flip
    get() = when (this) {
        Direction.Left -> Direction.Right
        Direction.Right -> Direction.Left
        Direction.Down -> Direction.Up
        Direction.Up -> Direction.Down
    }

fun <T> time(block: () -> T) {
    val start = System.currentTimeMillis()
    try {
        block.invoke()
    } finally {
        val end = System.currentTimeMillis()
        println("Took ${end - start} ms.")
    }
}

fun lcm(a: Long, b: Long) = a * b / gcd(a, b)

fun gcd(first: Long, second: Long): Long {
    var a = first
    var b = second

    while (a > 0 && b > 0) {
        if (a >= b) {
            a %= b
        } else {
            b %= a
        }
    }

    return maxOf(a, b)
}

interface Graph<T : Any> {
    fun neighborsOf(node: T): Iterable<Pair<T, Int>>
}

data class SearchResult<T : Any>(
    val startedFrom: T,
    val destination: T?,
    val searchTree: Map<T, Pair<T, Int>>,
)

data class SearchPath<T : Any>(
    val path: List<T>,
    val cost: Int,
) : List<T> by path

fun <T : Any> SearchResult<T>.pathTo(node: T): SearchPath<T>? {
    val cost = searchTree[node]?.second ?: return null
    val path = buildList {
        var current = node
        while (true) {
            add(current)
            val previous = searchTree.getValue(current).first
            if (previous == current) break
            current = previous
        }
    }.asReversed()
    return SearchPath(path, cost)
}

fun <T : Any> SearchResult<T>.path(): SearchPath<T>? = when (destination) {
    null -> null
    else -> pathTo(destination)
}

fun <T : Any> Graph<T>.search(
    start: T,
    maximumCost: Int = Int.MAX_VALUE,
    onVisited: (T) -> Unit = {},
    heuristic: (T) -> Int = { 0 },
    goalFunction: (T) -> Boolean = { false },
): SearchResult<T> {
    val queue = PriorityQueue(compareBy<Pair<T, Int>> { it.second })
    queue.add(start to 0)
    val searchTree = mutableMapOf(start to (start to 0))

    while (true) {
        val (node, costSoFar) = queue.poll() ?: return SearchResult(start, null, searchTree)
        onVisited(node)

        if (goalFunction(node)) return SearchResult(start, node, searchTree)

        neighborsOf(node).filter { it.first !in searchTree }.forEach { (next, cost) ->
            val nextCost = costSoFar + cost
            if (nextCost <= maximumCost && nextCost <= (searchTree[next]?.second ?: Int.MAX_VALUE)) {
                queue.add(next to heuristic(next) + nextCost)
                searchTree[next] = node to nextCost
            }
        }
    }
}