import kotlin.io.path.Path
import kotlin.io.path.readLines

fun readInput(name: String) = Path("src/$name.txt").readLines()
data class Vector2D(val x: Int, val y: Int)

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