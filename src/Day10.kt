fun main() {
    val d = Day10(readInput("input"))
    d.solve()
}

private class Day10(private val input: List<String>) : Day {
    override fun solvePart1() = parseInput(input.joinToString("\n")).loop.size / 2
    override fun solvePart2() = parseInput(input.joinToString("\n")).loopVolume

    private fun parseInput(input: String) = parse {
        lateinit var start: Vector2D

        fun parseNode(x: Int, y: Int, value: Char) = Pipe(
            location = Vector2D(x, y),
            dir = when (value) {
                '|' -> Direction.Up to Direction.Down
                '-' -> Direction.Left to Direction.Right
                'L' -> Direction.Up to Direction.Right
                'J' -> Direction.Up to Direction.Left
                '7' -> Direction.Down to Direction.Left
                'F' -> Direction.Down to Direction.Right
                else -> error("Unknown pipe type: $value")
            },
        )

        val rows = input.lines()
        val cols = rows.first().length
        require(rows.all { it.length == cols }) { "All rows must have the same length" }

        val area = Region2D(0 until cols, rows.indices)
        val pipes =
            rows.asReversed().asSequence().flatMapIndexed { y, line -> line.mapIndexed { x, c -> Triple(x, y, c) } }
                .onEach { (x, y, c) -> if (c == 'S') start = Vector2D(x, y) }.filterNot { it.third in ".S" }
                .map { (x, y, c) -> parseNode(x, y, c) }.associateBy { it.location }

        val startPipe = listOf(
            Direction.Up, Direction.Down, Direction.Left, Direction.Right
        ).filter { pipes[start.move(it)]?.redirect(it.flip) != null }
            .also { require(it.size == 2) { "Cannot determine start pipe type" } }.let { (i, o) -> Pipe(start, i to o) }

        PipeGrid(area, pipes.plus(start to startPipe), start)
    }
}

private data class Pipe(val location: Vector2D, val dir: Pair<Direction, Direction>) {
    fun redirect(inc: Direction) = when (inc) {
        dir.first -> dir.second
        dir.second -> dir.first
        else -> null
    }
}

private data class PipeGrid(
    private val area: Region2D, private val nodes: Map<Vector2D, Pipe>, private val start: Vector2D
) {
    val loop = buildList {
        fun Pair<Pipe, Direction>.flow(): Pair<Pipe, Direction> {
            val (node, dir) = this
            val nextNode = nodes[node.location.move(dir)]
            checkNotNull(nextNode) { "No next node at ${node.location.move(dir)}" }
            val nextFlow = nextNode.redirect(dir.flip)
            checkNotNull(nextFlow) { "No next flow at ${node.location.move(dir)}" }
            return nextNode to nextFlow
        }

        var pipe = nodes.getValue(start).let { it to it.dir.first }

        do {
            add(pipe.first)
            pipe = pipe.flow()
            if (size > nodes.size) error("Loop is too long")
        } while (pipe.first.location != start)
    }

    val loopVolume = run {
        val loopNodes = loop.toSet()
        var capacity = 0
        for (y in area.yRange) {
            var inside = false
            for (x in area.xRange) {
                val node = nodes[Vector2D(x, y)]
                val nodeInLoop = node in loopNodes
                val nodeIsVertical = nodeInLoop && node!!.dir.first == Direction.Up

                if (nodeIsVertical) inside = !inside
                if (!nodeInLoop && inside) capacity++
            }
        }
        capacity
    }
}