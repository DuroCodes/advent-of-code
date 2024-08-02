fun main() {
    val d = Day19(readInput("input"))
    d.solve()
}

private class Day19(input: List<String>) : Day {
    private val inputStr = input.joinToString("\n")
    private val workflows = inputStr.substringBefore("\n\n").lines().map { Workflow.from(it) }.associateBy { it.name }
    private val ratings = inputStr.substringAfter("\n\n").lines().map { Rating.from(it) }

    private fun Rating.score(workflow: Workflow): Int {
        val rule = workflow.rules.first { it.matches(this) }
        return when (rule.result) {
            "R" -> 0
            "A" -> categories.values.sum()
            else -> score(workflows.getValue(rule.result))
        }
    }

    private data class Rating(val categories: Map<Char, Int>) {
        companion object {
            fun from(str: String) = Rating(str.drop(1).dropLast(1).split(",").associate {
                it.substringBefore("=").single() to it.substringAfter("=").toInt()
            })
        }
    }

    private data class Workflow(val name: String, val rules: List<Rule>) {
        companion object {
            fun from(str: String): Workflow {
                val name = str.substringBefore("{")
                val rules = str.substringAfter("{").substringBefore("}").split(",").map { Rule.from(it) }
                return Workflow(name, rules)
            }
        }
    }

    private sealed class Rule {
        abstract val result: String

        data class Conditional(val lhs: Char, val op: Char, val rhs: Int, override val result: String) : Rule() {
            fun range() = if (op == '<') 1 until rhs else rhs + 1..4000
            fun reversedRange() = if (op == '<') rhs..4000 else 1..rhs
        }

        data class Unconditional(override val result: String) : Rule()

        fun matches(rating: Rating) = when (this) {
            is Unconditional -> true
            is Conditional -> when (op) {
                '<' -> rating.categories[lhs]!! < rhs
                '>' -> rating.categories[lhs]!! > rhs
                else -> error("Unknown operator: $op")
            }
        }

        companion object {
            fun from(str: String): Rule {
                return if (':' in str) {
                    val condition = str.substringBefore(":")
                    val result = str.substringAfter(":")
                    Conditional(condition[0], condition[1], condition.substring(2).toInt(), result)
                } else {
                    Unconditional(str)
                }
            }
        }
    }

    private fun IntRange.size() = last - start + 1
    private fun IntRange.merge(other: IntRange) = (maxOf(first, other.first)..minOf(last, other.last))

    private fun combinations(res: String, ranges: Map<Char, IntRange>): Long = when (res) {
        "R" -> 0
        "A" -> ranges.values.map { it.size().toLong() }.reduce(Long::times)
        else -> {
            val newRanges = ranges.toMutableMap()

            workflows.getValue(res).rules.sumOf { r ->
                when (r) {
                    is Rule.Unconditional -> combinations(r.result, newRanges)
                    is Rule.Conditional -> {
                        val newRange = newRanges.getValue(r.lhs).merge(r.range())
                        val newReversed = newRanges.getValue(r.lhs).merge(r.reversedRange())

                        newRanges[r.lhs] = newRange
                        combinations(r.result, newRanges).also { newRanges[r.lhs] = newReversed }
                    }
                }
            }
        }
    }

    override fun solvePart1() = ratings.sumOf { it.score(workflows.getValue("in")) }
    override fun solvePart2() = combinations("in", listOf('x', 'm', 'a', 's').associateWith { (1..4000) })
}