type answer = Answer of int | Unknown

let answer_to_string = function
  | Answer x -> Int.to_string x
  | Unknown -> "Not Yet Implemented"

module type Day = sig
  type input

  val parse_input : string -> input
  val solve_part1 : input -> answer
  val solve_part2 : input -> answer
  val part1 : string -> string
  val part2 : string -> string
end
