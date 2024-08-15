open Core
open Utils

module Day09 : Day = struct
  type direction = Up | Down | Left | Right
  type position = int * int
  type rope = { knots : position list }
  type input = Input of direction list

  let parse_line l =
    let split = String.split l ~on:' ' in
    let dir =
      match List.hd_exn split with
      | "U" -> Up
      | "D" -> Down
      | "L" -> Left
      | "R" -> Right
      | _ -> failwith "Invalid direction"
    in

    List.nth_exn split 1 |> Int.of_string |> List.init ~f:(fun _ -> dir)

  let parse_input t =
    t |> String.split_lines |> List.map ~f:parse_line |> List.concat |> fun x ->
    Input x

  let move_head (x, y) = function
    | Up -> (x, y + 1)
    | Down -> (x, y - 1)
    | Left -> (x - 1, y)
    | Right -> (x + 1, y)

  let move_tail (xh, yh) (xt, yt) =
    let sign x = Int.compare x 0 in
    let dx, dy = (xh - xt, yh - yt) in
    match (abs dx, abs dy) with
    | 0, 0 | 0, 1 | 1, 0 | 1, 1 -> (xt, yt)
    | _ -> (xt + sign dx, yt + sign dy)

  let move_rope { knots } direction =
    let rec move_knots acc knot = function
      | [] -> List.rev acc
      | nk :: nks ->
          let new_pos = move_tail knot nk in
          move_knots (new_pos :: acc) new_pos nks
    in
    let head, tail = (List.hd_exn knots, List.tl_exn knots) in
    let new_head = move_head head direction in

    { knots = new_head :: move_knots [] new_head tail }

  let compare_pos (x1, y1) (x2, y2) =
    match (x2 - x1, y2 - y1) with 0, dy -> dy | dx, _ -> dx

  let solve input init_length =
    input
    |> List.fold
         ~init:[ { knots = List.init init_length ~f:(fun _ -> (0, 0)) } ]
         ~f:(fun ropes dir -> move_rope (List.hd_exn ropes) dir :: ropes)
    |> List.map ~f:(fun { knots } -> List.last_exn knots)
    |> List.dedup_and_sort ~compare:compare_pos
    |> List.length
    |> fun x -> AnswerInt x

  let solve_part1 (Input input) = solve input 2
  let solve_part2 (Input input) = solve input 10

  let part1 input_str =
    input_str |> parse_input |> solve_part1 |> answer_to_string

  let part2 input_str =
    input_str |> parse_input |> solve_part2 |> answer_to_string
end
