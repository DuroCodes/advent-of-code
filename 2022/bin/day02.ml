open Core
open Utils

module Day02 : Day = struct
  type move = Rock | Paper | Scissors
  type outcome = Win | Loss | Draw
  type strategy1 = { opp : move; player : move }
  type strategy2 = { opp : move; res : outcome }
  type input = Input of (strategy1 list * strategy2 list)

  let part1_parse_line str =
    let opponent, player = (String.get str 0, String.get str 2) in
    let parse c =
      match c with
      | 'A'
      | 'X' ->
          Rock
      | 'B'
      | 'Y' ->
          Paper
      | 'C'
      | 'Z' ->
          Scissors
      | _ -> failwith "Invalid move"
    in
    { opp = parse opponent; player = parse player }

  let part2_parse_line str =
    let opponent, outcome = (String.get str 0, String.get str 2) in
    let parse_opponent c =
      match c with
      | 'A' -> Rock
      | 'B' -> Paper
      | 'C' -> Scissors
      | _ -> failwith "Invalid move"
    and parse_outcome c =
      match c with
      | 'X' -> Loss
      | 'Y' -> Draw
      | 'Z' -> Win
      | _ -> failwith "Invalid outcome"
    in
    { opp = parse_opponent opponent; res = parse_outcome outcome }

  let parse_input t =
    t |> String.split_lines
    |> List.filter ~f:(String.( <> ) "")
    |> List.map ~f:(fun x -> (part1_parse_line x, part2_parse_line x))
    |> List.unzip
    |> fun x -> Input x

  let move_score = function
    | Rock -> 1
    | Paper -> 2
    | Scissors -> 3

  let outcome_score x =
    match x with
    | Loss -> 0
    | Draw -> 3
    | Win -> 6

  let round_outcome opponent player =
    match (opponent, player) with
    | Rock, Paper
    | Paper, Scissors
    | Scissors, Rock ->
        Win
    | Rock, Rock
    | Paper, Paper
    | Scissors, Scissors ->
        Draw
    | _ -> Loss

  let round_score opponent player =
    (round_outcome opponent player |> outcome_score) + move_score player

  let player_move opponent outcome =
    match (opponent, outcome) with
    | Rock, Loss
    | Paper, Win
    | Scissors, Draw ->
        Scissors
    | Rock, Draw
    | Paper, Loss
    | Scissors, Win ->
        Rock
    | _ -> Paper

  let solve_part1 (Input (input, _)) =
    input
    |> List.map ~f:(fun { opp; player } -> round_score opp player)
    |> List.fold ~init:0 ~f:( + )
    |> fun x -> AnswerInt x

  let solve_part2 (Input (_, input)) =
    input
    |> List.map ~f:(fun { opp; res } -> round_score opp (player_move opp res))
    |> List.fold ~init:0 ~f:( + )
    |> fun x -> AnswerInt x

  let part1 input_str = input_str |> parse_input |> solve_part1 |> answer_to_string
  let part2 input_str = input_str |> parse_input |> solve_part2 |> answer_to_string
end
