open Base

type answer = AnswerInt of int | AnswerCharList of char list | Unknown

let answer_to_string = function
  | AnswerInt x -> Int.to_string x
  | AnswerCharList x -> String.of_char_list x
  | Unknown -> "Not Yet Implemented"

module type Day = sig
  type input

  val parse_input : string -> input
  val solve_part1 : input -> answer
  val solve_part2 : input -> answer
  val part1 : string -> string
  val part2 : string -> string
end
