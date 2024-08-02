fun main() {
    val d = Day24(readInput("input"))
    d.solve()
}

private class Day24(input: List<String>) : Day {
    val parsed = input.map {
        val lineRegex = Regex("""^(-?\d+), (-?\d+), (-?\d+) @ (-?\d+), (-?\d+), (-?\d+)$""")
        lineRegex.matchEntire(it)!!.destructured
    }.map { (x, y, z, vx, vy, vz) ->
        HailStone(x.toLong(), y.toLong(), z.toLong(), vx.toLong(), vy.toLong(), vz.toLong())
    }

    override fun solvePart1(): Int {
        val area = if (parsed.size < 10) 7.0..27.0 else 200000000000000.0..400000000000000.0

        return parsed.distinctPairs().mapNotNull { (first, second) -> intersections(first, second) }
            .count { (x, y) -> x in area && y in area }
    }

    override fun solvePart2(): Long {
        val (first, second) = parsed
        val amp = if (parsed.size < 10) 5 else 250

        return possibleRockVels(parsed, amp).mapNotNull { v ->
            deduceThrowLoc(first, second, v)?.let {
                HailStone(
                    it, v
                )
            }
        }.first { r -> parsed.all { r.willCollide(it) } }.let { it.x + it.y + it.z }
    }

    private fun intersections(first: HailStone, second: HailStone): Pair<Double, Double>? {
        val c1 = (first.vy * first.x - first.vx * first.y).toDouble()
        val c2 = (second.vy * second.x - second.vx * second.y).toDouble()

        val slopeDiff = first.vy * -second.vx - second.vy * -first.vx
        if (slopeDiff == 0L) return null

        val x = (c1 * -second.vx - c2 * -first.vx) / slopeDiff
        val y = (c1 * -second.vy - c2 * -first.vy) / slopeDiff

        val futureIntersects = listOf(
            (x - first.x < 0) == (first.vx < 0),
            (y - first.y < 0) == (first.vy < 0),
            (x - second.x < 0) == (second.vx < 0),
            (y - second.y < 0) == (second.vy < 0)
        ).all { it }

        return (x to y).takeIf { futureIntersects }
    }

    private fun possibleRockVels(hailstones: List<HailStone>, amplitude: Int) = sequence {
        val velRange = -amplitude..amplitude
        val invalidXRanges = mutableSetOf<LongRange>()
        val invalidYRanges = mutableSetOf<LongRange>()
        val invalidZRanges = mutableSetOf<LongRange>()

        fun testImpossible(set: MutableSet<LongRange>, p0: Long, v0: Long, p1: Long, v1: Long) {
            if (p0 > p1 && v0 > v1) set.add(v1..v0)
            if (p0 < p1 && v0 < v1) set.add(v0..v1)
        }

        for ((first, second) in hailstones.distinctPairs()) {
            testImpossible(invalidXRanges, first.x, first.vx, second.x, second.vx)
            testImpossible(invalidYRanges, first.y, first.vy, second.y, second.vy)
            testImpossible(invalidZRanges, first.z, first.vz, second.z, second.vz)
        }

        val possibleX = velRange.filter { x -> invalidXRanges.none { x in it } }
        val possibleY = velRange.filter { y -> invalidYRanges.none { y in it } }
        val possibleZ = velRange.filter { z -> invalidZRanges.none { z in it } }

        for (vx in possibleX) {
            for (vy in possibleY) {
                for (vz in possibleZ) {
                    yield(Vector3D(vx, vy, vz))
                }
            }
        }
    }

    private fun deduceThrowLoc(first: HailStone, second: HailStone, vel: Vector3D): Vector3D? {
        val firstRelX = first.vx - vel.x
        val firstRelY = first.vy - vel.y
        val secondRelX = second.vx - vel.x
        val secondRelY = second.vy - vel.y

        val slopeDiff = firstRelX * secondRelY - firstRelY * secondRelX
        if (slopeDiff == 0L) return null

        val t = (secondRelY * (second.x - first.x) - secondRelX * (second.y - first.y)) / slopeDiff
        if (t < 0) return null

        return Vector3D(
            first.x + (first.vx - vel.x) * t,
            first.y + (first.vy - vel.y) * t,
            first.z + (first.vz - vel.z) * t,
        )
    }
}

private data class HailStone(val x: Long, val y: Long, val z: Long, val vx: Long, val vy: Long, val vz: Long) {
    constructor(pos: Vector3D, vel: Vector3D) : this(pos.x, pos.y, pos.z, vel.x, vel.y, vel.z)

    fun posAfterTime(time: Double): Triple<Double, Double, Double> = Triple(
        x + vx * time, y + vy * time, z + vz * time
    )

    fun willCollide(other: HailStone): Boolean {
        val t = when {
            vx != other.vx -> (other.x - x).toDouble() / (vx - other.vx)
            vy != other.vy -> (other.y - y).toDouble() / (vy - other.vy)
            vz != other.vz -> (other.z - z).toDouble() / (vz - other.vz)
            else -> return false
        }

        return if (t < 0) false else posAfterTime(t) == other.posAfterTime(t)
    }
}

