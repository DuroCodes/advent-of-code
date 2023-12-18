fun main() {
    val d = Day17(readInput("input"))
    d.solve()
}

private class Day17(input: List<String>) : Day {
    val parsedInput = input.asReversed().flatMapIndexed { y, l ->
        l.mapIndexed { x, n -> Vector2D(x, y) to n.digitToInt() }
    }.let(::GearIslandMap)

    class GearIslandMap(initial: Iterable<Pair<Vector2D<Int>, Int>>) {
        private val grid: Map<Vector2D<Int>, Int> = initial.associate { it.first to it.second }

        val area = Region2D(grid.keys)

        private data class CrucibleSearchState(
            val position: Vector2D<Int>,
            val direction: Direction,
            val straightLineStreak: Int,
        )

        private fun neighborsOf(
            node: CrucibleSearchState,
            ultraCrucible: Boolean,
        ): List<Pair<CrucibleSearchState, Int>> {
            val validDirections =
                listOf(Direction.Up, Direction.Down, Direction.Left, Direction.Right).minus(node.direction.flip)
                    .run { if (node.straightLineStreak == if (ultraCrucible) 10 else 3) minus(node.direction) else this }
                    .run { if (ultraCrucible && node.straightLineStreak in 1..<4) listOf(node.direction) else this }
                    .filter { direction -> node.position.move(direction) in area }

            return validDirections.map { direction ->
                val nextPosition = node.position.move(direction)
                val nextCost = grid.getValue(nextPosition)
                val nextState = CrucibleSearchState(
                    position = nextPosition,
                    direction = direction,
                    straightLineStreak = if (node.direction == direction) node.straightLineStreak.inc() else 1,
                )
                nextState to nextCost
            }
        }

        fun navigate(start: Vector2D<Int>, end: Vector2D<Int>, withUltraCrucible: Boolean): List<Pair<Vector2D<Int>, Int>> {
            val graph = object : Graph<CrucibleSearchState> {
                override fun neighborsOf(node: CrucibleSearchState) = neighborsOf(node, withUltraCrucible)
            }

            val search = graph.search(
                start = CrucibleSearchState(start, Direction.Right, 0),
                goalFunction = { it.position == end && (!withUltraCrucible || it.straightLineStreak >= 4) },
            )

            return search.path()!!.map { it.position to search.searchTree[it]!!.second }
        }

        fun solve(withUltraCrucible: Boolean = false): Int {
            val start = Vector2D(area.xRange.first, area.yRange.last)
            val end = Vector2D(area.xRange.last, area.yRange.first)
            return navigate(start, end, withUltraCrucible).last().second
        }
    }

    override fun solvePart1() = parsedInput.solve()
    override fun solvePart2() = parsedInput.solve(true)
}