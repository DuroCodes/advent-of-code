open Core
open Utils

module Day04 : Day = struct
  type assignment = int * int
  type input = Input of (assignment * assignment) list

  let parse_argument s =
    let parts = String.split s ~on:'-' in
    (Int.of_string @@ List.hd_exn parts, Int.of_string @@ List.last_exn parts)

  let fully_covers (a1, a2) (b1, b2) = a1 <= b1 && a2 >= b2
  let overlaps (a1, a2) (b1, b2) = a1 <= b2 && a2 >= b1

  let parse_input t =
    t |> String.split_lines
    |> List.map ~f:(String.split ~on:',')
    |> List.map ~f:(fun x -> List.map x ~f:parse_argument)
    |> List.map ~f:(fun x -> (List.hd_exn x, List.last_exn x))
    |> fun x -> Input x

  let solve_part1 (Input input) =
    input
    |> List.filter ~f:(fun (x, y) -> fully_covers x y || fully_covers y x)
    |> List.length
    |> fun x -> AnswerInt x

  let solve_part2 (Input input) =
    input |> List.filter ~f:(fun (x, y) -> overlaps x y) |> List.length
    |> fun x -> AnswerInt x

  let part1 input_str =
    input_str |> parse_input |> solve_part1 |> answer_to_string

  let part2 input_str =
    input_str |> parse_input |> solve_part2 |> answer_to_string
end
