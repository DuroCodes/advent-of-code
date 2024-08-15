open Core
open Utils

module Day14 : Day = struct
  type element = Air | Rock | Sand
  type cave_map = element array
  type input = Input of cave_map

  let max_x, max_y = (1_000, 200)
  let position_to_idx (x, y) = (y * max_x) + x
  let idx_to_position idx = (idx % max_x, idx / max_x)

  let check_points line =
    line |> String.split ~on:' '
    |> List.filter ~f:(fun x -> String.( <> ) x "->")
    |> List.map ~f:(fun x -> String.split x ~on:',')
    |> List.map ~f:(fun x ->
           (Int.of_string (List.nth_exn x 0), Int.of_string (List.nth_exn x 1)))
    |> List.map ~f:position_to_idx

  let build_path prev_idx next_idx =
    let (x1, y1), (x2, y2) =
      (idx_to_position prev_idx, idx_to_position next_idx)
    in
    let dx, dy = (x2 - x1, y2 - y1) in
    let sign x = Int.compare x 0 in
    let path_pos =
      match (dx, dy) with
      | 0, 0 -> []
      | 0, dy -> List.init (abs dy) ~f:(fun n -> (x1, ((n + 1) * sign dy) + y1))
      | dx, 0 -> List.init (abs dx) ~f:(fun n -> (((n + 1) * sign dx) + x1, y1))
      | _ -> failwith "Invalid path"
    in
    List.map path_pos ~f:position_to_idx

  let line_trajectory line =
    line |> check_points
    |> List.fold ~init:[] ~f:(fun path next_idx ->
           match path with
           | [] -> [ next_idx ]
           | prev_idx :: _ ->
               List.rev_append (build_path prev_idx next_idx) path)

  let parse_input t =
    let rocks = t |> String.split_lines |> List.concat_map ~f:line_trajectory in
    let arr = Array.init (max_x * max_y) ~f:(fun _ -> Air) in
    List.iter rocks ~f:(fun idx -> Array.set arr idx Rock);
    Input arr

  let place_for_sand cave start_idx =
    let lookup idx = Array.get cave idx in
    let rec iter idx =
      let x, y = idx_to_position idx in
      let down = position_to_idx (x, y + 1) in
      let down_left = position_to_idx (x - 1, y + 1) in
      let down_right = position_to_idx (x + 1, y + 1) in

      if y + 1 >= max_y || x < 0 || x >= max_x then None
      else
        match (lookup down_left, lookup down, lookup down_right) with
        | _, Air, _ -> iter down
        | Air, _, _ -> iter down_left
        | _, _, Air -> iter down_right
        | _ -> Some idx
    in
    iter start_idx

  let fill_cave cave start_idx =
    let rec iter cave =
      match place_for_sand cave start_idx with
      | None -> cave
      | Some idx ->
          Array.set cave idx Sand;
          if idx = start_idx then cave else iter cave
    in
    iter cave

  let solve_part1 (Input input) =
    let start_idx = 500 in
    fill_cave input start_idx
    |> Array.filter ~f:(function Sand -> true | _ -> false)
    |> Array.length
    |> fun x -> AnswerInt x

  let solve_part2 (Input input) =
    let start_idx = 500 in
    let max_y_of_rock =
      input
      |> Array.filter_mapi ~f:(fun i x ->
             match x with Rock -> Some i | _ -> None)
      |> Array.fold ~init:0 ~f:(fun z i ->
             let _, y = idx_to_position i in
             max z y)
    in
    let endless_floor =
      Array.init max_x ~f:(fun x -> position_to_idx (x, max_y_of_rock + 2))
    in
    let cave =
      Array.iter endless_floor ~f:(fun i -> Array.set input i Rock);
      fill_cave input start_idx
    in

    cave
    |> Array.filter ~f:(function Sand -> true | _ -> false)
    |> Array.length
    |> fun x -> AnswerInt x

  let part1 input_str =
    input_str |> parse_input |> solve_part1 |> answer_to_string

  let part2 input_str =
    input_str |> parse_input |> solve_part2 |> answer_to_string
end
