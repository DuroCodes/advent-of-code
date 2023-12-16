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

data class Region2D(val xRange: IntRange, val yRange: IntRange) : Iterable<Vector2D> {
    fun contains(point: Vector2D) = point.x in xRange && point.y in yRange
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

data class Vector2D(val x: Int, val y: Int) {
    fun move(dir: Direction) = when (dir) {
        Direction.Left -> Vector2D(x - 1, y)
        Direction.Right -> Vector2D(x + 1, y)
        Direction.Up -> Vector2D(x, y + 1)
        Direction.Down -> Vector2D(x, y - 1)
    }

    operator fun plus(other: Vector2D) = Vector2D(x + other.x, y + other.y)
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