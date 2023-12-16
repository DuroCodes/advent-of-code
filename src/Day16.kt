fun main() {
    val d = Day16(readInput("input"))
    d.solve()
}

private class Day16(private val input: List<String>) : Day {
    private val obstacles = buildList {
        for (x in input.indices) {
            for (y in input[x].indices) {
                val char = input[x][y]
                if (char != '.') {
                    add(Obstacle(Vector2D(x, y), type = char))
                }
            }
        }
    }

    val startingBeams = buildList {
        for (x in input.indices) {
            add(Beam(Vector2D(x, -1), Vector2D(0, 1)))
            add(Beam(Vector2D(x, input[x].length), Vector2D(0, -1)))
        }
        for (y in input.first().indices) {
            add(Beam(Vector2D(-1, y), Vector2D(1, 0)))
            add(Beam(Vector2D(input.size, y), Vector2D(-1, 0)))
        }
    }

    override fun solvePart1() = energizedTiles(Beam(Vector2D(0, -1), Vector2D(0, 1)))

    override fun solvePart2() = startingBeams.maxOf { energizedTiles(it) }

    private fun energizedTiles(b: Beam): Int {
        val visited = mutableSetOf<Beam>()

        val queue = ArrayDeque<Beam>()
        queue += b

        while (queue.isNotEmpty()) {
            val beam = queue.removeFirst()

            val newPos = beam.pos + beam.dir

            if (newPos.x !in input.indices || newPos.y !in input.first().indices) continue

            val newDirs = obstacles.firstOrNull { obstacle -> obstacle.pos == newPos }?.let { (_, character) ->
                buildList {
                    when (character) {
                        '/' -> add(Vector2D(-beam.dir.y, -beam.dir.x))
                        '\\' -> add(Vector2D(beam.dir.y, beam.dir.x))
                        '|' -> {
                            add(Vector2D(-beam.dir.y, 0))
                            add(Vector2D(beam.dir.y, 0))
                            add(Vector2D(beam.dir.x, 0))
                        }

                        '-' -> {
                            add(Vector2D(0, -beam.dir.x))
                            add(Vector2D(0, beam.dir.x))
                            add(Vector2D(0, beam.dir.y))
                        }
                    }
                }
            }?.filterNot { it == Vector2D(0, 0) } ?: listOf(beam.dir)

            newDirs.forEach { d ->
                Beam(newPos, d).takeUnless { it in visited }?.also {
                    queue.add(it)
                    visited.add(it)
                }
            }

        }

        return visited.map { it.pos }.toSet().size
    }
}

data class Beam(val pos: Vector2D, val dir: Vector2D)
data class Obstacle(val pos: Vector2D, val type: Char)