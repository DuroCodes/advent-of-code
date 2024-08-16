open Core
open Utils

module Day10 : Day = struct
  type register = int
  type history = register list
  type instruction = Noop | Addx of int
  type input = Input of instruction list
  type accumulator = { v : register; h : history }

  let parse_line l =
    match String.split l ~on:' ' with
    | "noop" :: _ -> Noop
    | "addx" :: x :: _ -> Addx (Int.of_string x)
    | _ -> failwith "Invalid instruction"

  let parse_input t = t |> String.split_lines |> List.map ~f:parse_line |> fun x -> Input x

  let process_instructions instructions =
    let initial = { v = 1; h = [ 1 ] } in
    let rec iterate acc = function
      | [] -> acc
      | Noop :: is -> iterate { v = acc.v; h = acc.v :: acc.h } is
      | Addx value :: is -> iterate { v = acc.v + value; h = acc.v :: acc.v :: acc.h } is
    in
    let { h; _ } = iterate initial instructions in
    List.rev h

  let solve_part1 (Input input) =
    input |> process_instructions
    |> List.filter_mapi ~f:(fun i x -> if (i - 20) % 40 = 0 then Some (x * i) else None)
    |> List.fold ~init:0 ~f:( + )
    |> fun x -> AnswerInt x

  let solve_part2 (Input input) =
    input |> process_instructions |> fun x ->
    List.drop x 1
    |> List.mapi ~f:(fun i x -> if abs (x - (i % 40)) <= 1 then '#' else ' ')
    |> List.chunks_of ~length:40 |> List.map ~f:String.of_char_list |> String.concat ~sep:"\n"
    |> fun x -> AnswerString x

  let part1 input_str = input_str |> parse_input |> solve_part1 |> answer_to_string
  let part2 input_str = input_str |> parse_input |> solve_part2 |> answer_to_string
end
