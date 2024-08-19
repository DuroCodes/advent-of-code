open Core
open Utils

module Day18 : Day = struct
  type lava_cube = int * int * int
  type lava_droplet = { surface : int; cubes : lava_cube list }
  type input = Input of lava_cube list

  let adjacent_cubes = [ (-1, 0, 0); (1, 0, 0); (0, -1, 0); (0, 1, 0); (0, 0, -1); (0, 0, 1) ]

  let parse_line l =
    match String.split l ~on:',' with
    | x :: y :: z :: _ -> (Int.of_string x, Int.of_string y, Int.of_string z)
    | _ -> failwith "Invalid cube"

  let parse_input t = t |> String.split_lines |> List.map ~f:parse_line |> fun x -> Input x

  let grow_droplet { surface; cubes } (x, y, z) =
    let adjacent_positions =
      List.map adjacent_cubes ~f:(fun (dx, dy, dz) -> (x + dx, y + dy, z + dz))
    in
    let adjacent_count =
      List.count adjacent_positions ~f:(fun (x1, y1, z1) ->
          List.exists cubes ~f:(fun (x2, y2, z2) -> x1 = x2 && y1 = y2 && z1 = z2))
    in

    let new_surface = surface + 6 - (2 * adjacent_count) in
    { surface = new_surface; cubes = (x, y, z) :: cubes }

  let discover_outside_cubes (min_x, min_y, min_z) (max_x, max_y, max_z) cubes =
    let valid_point (x, y, z) discovered =
      x >= min_x && x <= max_x && y >= min_y && y <= max_y && z >= min_z && z <= max_z
      && (not (List.exists discovered ~f:(fun (x1, y1, z1) -> x = x1 && y = y1 && z = z1)))
      && not (List.exists cubes ~f:(fun (x1, y1, z1) -> x = x1 && y = y1 && z = z1))
    in

    let rec discover_outside_cubes' discovered (x, y, z) =
      let new_cubes =
        adjacent_cubes
        |> List.filter_map ~f:(fun (dx, dy, dz) ->
               let p = (x + dx, y + dy, z + dz) in
               if valid_point p discovered then Some p else None)
      in
      List.fold new_cubes ~init:(discovered @ new_cubes) ~f:discover_outside_cubes'
    in
    discover_outside_cubes' [] (0, 0, 0)

  let solve_part1 (Input input) =
    let init_droplet = { surface = 0; cubes = [] } in
    let final_droplet = List.fold input ~init:init_droplet ~f:grow_droplet in
    AnswerInt final_droplet.surface

  let solve_part2 (Input input) =
    let get_max ~f =
      input |> List.map ~f |> List.max_elt ~compare:Int.compare |> function
      | Some x -> x
      | _ -> failwith "Invalid input"
    in
    let get_min ~f =
      input |> List.map ~f |> List.min_elt ~compare:Int.compare |> function
      | Some x -> x
      | _ -> failwith "Invalid input"
    in

    let min_x = get_min ~f:(fun (x, _, _) -> x) - 1 in
    let max_x = get_max ~f:(fun (x, _, _) -> x) + 1 in
    let min_y = get_min ~f:(fun (_, y, _) -> y) - 1 in
    let max_y = get_max ~f:(fun (_, y, _) -> y) + 1 in
    let min_z = get_min ~f:(fun (_, _, z) -> z) - 1 in
    let max_z = get_max ~f:(fun (_, _, z) -> z) + 1 in

    let outside_cube_surface =
      ((max_x - min_x + 1) * (max_y - min_y + 1) * 2)
      + ((max_y - min_y + 1) * (max_z - min_z + 1) * 2)
      + ((max_z - min_z + 1) * (max_x - min_x + 1) * 2)
    in

    let outside_cubes = discover_outside_cubes (min_x, min_y, min_z) (max_x, max_y, max_z) input in

    let outside_total_surface =
      outside_cubes |> List.fold ~init:{ surface = 0; cubes = [] } ~f:grow_droplet |> fun x ->
      x.surface
    in

    AnswerInt (outside_total_surface - outside_cube_surface)

  let part1 input_str = input_str |> parse_input |> solve_part1 |> answer_to_string
  let part2 input_str = input_str |> parse_input |> solve_part2 |> answer_to_string
end
