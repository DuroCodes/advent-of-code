fun main() {
    val d = Day23(readInput("input"))
    d.solve()
}

private class Day23(private val input: List<String>) : Day {
    fun parseInput(withSlopes: Boolean = true) =
        input.asReversed().flatMapIndexed { y, l -> l.mapIndexed { x, c -> Vector2D(x, y) to c } }
            .mapNotNull { (p, c) ->
                val slope = when (c) {
                    '>' -> Direction.Right
                    '<' -> Direction.Left
                    '^' -> Direction.Up
                    'v' -> Direction.Down
                    else -> null
                }.takeIf { withSlopes }
                Node(p, slope).takeUnless { c == '#' }
            }

    override fun solvePart1() = solveNodes(parseInput())
    override fun solvePart2() = solveNodes(parseInput(false))

    private fun solveNodes(nodes: List<Node>) = Navigator(nodes).scenicRouteLength(
        Navigator(nodes).start, Navigator(nodes).end
    )
}

private data class Node(val loc: Vector2D<Int>, val slope: Direction?)
private data class POI(val loc: Vector2D<Int>, val neighbors: Map<Vector2D<Int>, Int>)

private class Navigator(nodes: Iterable<Node>) {
    val start: Vector2D<Int>
    val end: Vector2D<Int>
    private val navMap: Map<Vector2D<Int>, POI>

    init {
        val originalNodes = nodes.associateBy { it.loc }
        val region = Region2D(originalNodes.keys)

        start = Vector2D(region.xRange.first, region.yRange.last)
        end = Vector2D(region.xRange.last, region.yRange.first)

        val points = buildSet {
            add(start)
            for (node in originalNodes.values) {
                val neighbors = listOf(
                    Direction.Up, Direction.Down, Direction.Left, Direction.Right
                ).map { node.loc.move(it) }.filter { it in originalNodes }

                if (neighbors.size >= 3) add(node.loc)
            }

            add(end)
        }

        fun walk(from: Vector2D<Int>, dir: Direction): Pair<Vector2D<Int>, Int>? {
            var trailLen = 0
            var trailDir = dir
            var trailLoc = from

            while (true) {
                trailLen += 1
                trailLoc = trailLoc.move(trailDir)

                when (trailLoc) {
                    !in originalNodes -> return null
                    in points -> return trailLoc to trailLen
                }

                trailDir = when (val slope = originalNodes.getValue(trailLoc).slope) {
                    null -> listOf(Direction.Up, Direction.Down, Direction.Left, Direction.Right).minus(trailDir.flip)
                        .firstOrNull { trailLoc.move(it) in originalNodes } ?: return null

                    else -> {
                        if (slope != trailDir) return null
                        slope
                    }
                }
            }
        }

        navMap = points.associateWith { poi ->
            listOf(Direction.Up, Direction.Down, Direction.Left, Direction.Right).mapNotNull { d -> walk(poi, d) }
                .toMap().let { POI(poi, it) }
        }
    }

    private fun dfsMaximize(loc: Vector2D<Int>, goal: Vector2D<Int>, seen: MutableSet<Vector2D<Int>>): Int {
        if (loc == goal) return 0
        var maxCost = Int.MIN_VALUE
        val node = navMap.getValue(loc)
        seen.add(loc)

        for (next in node.neighbors) {
            if (next.key in seen) continue
            maxCost = maxOf(maxCost, dfsMaximize(next.key, goal, seen) + node.neighbors.getValue(next.key))
        }

        seen.remove(loc)
        return maxCost
    }

    fun scenicRouteLength(start: Vector2D<Int>, end: Vector2D<Int>) = dfsMaximize(start, end, mutableSetOf())
}