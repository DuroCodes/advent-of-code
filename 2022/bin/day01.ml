open Base
open Utils

module Day01 : Day = struct
  type input = Input of int list list

  let get_int = function "" -> None | t -> Some (Int.of_string t)

  let parse_input t =
    t |> String.split_lines
    |> List.group ~break:(fun x _ -> String.(x = ""))
    |> List.map ~f:(fun x -> List.filter_map x ~f:get_int)
    |> fun x -> Input x

  let solve_part1 (Input input) =
    input
    |> List.map ~f:(List.fold ~init:0 ~f:( + ))
    |> List.max_elt ~compare:Int.compare
    |> fun x -> match x with None -> AnswerInt 0 | Some n -> AnswerInt n

  let solve_part2 (Input input) =
    input
    |> List.map ~f:(List.fold ~init:0 ~f:( + ))
    |> List.sort ~compare:Int.compare
    |> List.rev
    |> (fun x -> List.take x 3)
    |> List.fold ~init:0 ~f:( + )
    |> fun x -> AnswerInt x

  let part1 input_str =
    input_str |> parse_input |> solve_part1 |> answer_to_string

  let part2 input_str =
    input_str |> parse_input |> solve_part2 |> answer_to_string
end
