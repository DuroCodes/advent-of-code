open Core
open Utils

module Day06 : Day = struct
  type input = Input of char list
  type marker = Packet | Message

  let parse_input t = Input (String.to_list t)

  let identify_start marker chars =
    let len =
      match marker with
      | Packet -> 4
      | Message -> 14
    in
    let rec identify chars acc =
      match chars with
      | [] -> 0
      | _ ->
          let beg = List.take chars len in
          if List.length (List.dedup_and_sort ~compare:Char.compare beg) = len then acc + len
          else identify (List.drop chars 1) (acc + 1)
    in
    identify chars 0

  let solve_part1 (Input input) = AnswerInt (identify_start Packet input)
  let solve_part2 (Input input) = AnswerInt (identify_start Message input)
  let part1 input_str = input_str |> parse_input |> solve_part1 |> answer_to_string
  let part2 input_str = input_str |> parse_input |> solve_part2 |> answer_to_string
end
