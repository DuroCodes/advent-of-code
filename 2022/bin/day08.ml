open Base
open Utils

module Day08 : Day = struct
  type grid = int array
  type forest = { size : int; grid : grid }
  type input = Input of forest

  let parse_input t =
    let lines = String.split_lines t in
    let size = List.length lines in
    let grid =
      String.concat lines |> String.to_array
      |> Array.map ~f:(fun c -> Int.of_string (Char.to_string c))
    in

    Input { size; grid }

  let get_projections index { size; _ } =
    let index_to_coords index = (index / size, index % size) in
    let coord_to_index (x, y) = (x * size) + y in

    let x, y = index_to_coords index in
    let empty_grid = Array.init (size * size) ~f:index_to_coords in

    let grid_projections =
      [|
        Array.filter empty_grid ~f:(fun (i, j) -> i = x && j > y);
        Array.filter empty_grid ~f:(fun (i, j) -> i = x && j < y) |> Array.rev;
        Array.filter empty_grid ~f:(fun (i, j) -> i > x && j = y);
        Array.filter empty_grid ~f:(fun (i, j) -> i < x && j = y) |> Array.rev;
      |]
    in

    Array.map grid_projections ~f:(Array.map ~f:coord_to_index)

  let is_visible tree_idx forest =
    let grid = forest.grid in
    let projections = get_projections tree_idx forest in
    let tree_height = grid.(tree_idx) in

    Array.exists projections ~f:(fun p ->
        Array.for_all p ~f:(fun i -> grid.(i) < tree_height))

  let scenic_score tree_idx forest =
    let grid = forest.grid in
    let projections = get_projections tree_idx forest in

    let rec counter = function
      | [] -> 0
      | l :: ls -> if grid.(tree_idx) <= l then 1 else 1 + counter ls
    in

    Array.map projections ~f:(fun p -> Array.map p ~f:(fun i -> grid.(i)))
    |> Array.map ~f:(fun p -> counter (Array.to_list p))
    |> Array.fold ~init:1 ~f:( * )

  let solve_part1 (Input input) =
    input.grid
    |> Array.mapi ~f:(fun i _ -> is_visible i input)
    |> Array.filter ~f:Fn.id |> Array.length
    |> fun x -> AnswerInt x

  let solve_part2 (Input input) =
    input.grid
    |> Array.mapi ~f:(fun i _ -> scenic_score i input)
    |> Array.fold ~init:0 ~f:Int.max
    |> fun x -> AnswerInt x

  let part1 input_str =
    input_str |> parse_input |> solve_part1 |> answer_to_string

  let part2 input_str =
    input_str |> parse_input |> solve_part2 |> answer_to_string
end
