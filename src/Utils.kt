import kotlin.io.path.Path
import kotlin.io.path.readLines

fun readInput(name: String) = Path("src/$name.txt").readLines()

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

data class Vector2D(val x: Int, val y: Int) {
    fun move(dir: Direction) = when (dir) {
        Direction.Left -> Vector2D(x - 1, y)
        Direction.Right -> Vector2D(x + 1, y)
        Direction.Up -> Vector2D(x, y + 1)
        Direction.Down -> Vector2D(x, y - 1)
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