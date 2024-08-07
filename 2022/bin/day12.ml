open Base
open Utils

module Day12 : Day = struct
  type elevation = Start of int | End of int | Elevation of int
  type map = { height : int; width : int; elevations : elevation array }

  type rules = {
    map : map;
    if_move_valid : elevation -> elevation -> bool;
    if_goal_reached : elevation -> bool;
    start_idx : int;
  }

  type shortest_path = { map_idx : int; length : int }
  type input = Input of map
  type path = Path of int list

  let parse_input t =
    let lines = String.split_lines t in
    let height = List.length lines in
    let width = String.length (List.hd_exn lines) in
    let elevation = function
      | 'S' -> Start 0
      | 'E' -> End (Char.to_int 'z' - Char.to_int 'a')
      | x -> Elevation (Char.to_int x - Char.to_int 'a')
    in
    let elevations =
      String.concat lines ~sep:"" |> String.to_array |> Array.map ~f:elevation
    in
    Input { height; width; elevations }

  let is_shortest_path (Path path) paths =
    List.for_all paths ~f:(fun { map_idx; length } ->
        map_idx <> List.hd_exn path || length > List.length path)

  let add_path (Path path) shortest_paths =
    { map_idx = List.hd_exn path; length = List.length path }
    :: List.filter shortest_paths ~f:(fun sp -> sp.map_idx <> List.hd_exn path)

  let adjacent_positions idx height m_w =
    [
      (if idx % m_w = 0 then -1 else idx - 1);
      (if (idx + 1) % m_w = 0 then -1 else idx + 1);
      idx + m_w;
      idx - m_w;
    ]
    |> List.filter ~f:(fun x -> x >= 0 && x < m_w * height)

  let discover_paths { map = { height; width; elevations }; if_move_valid; _ }
      discovered =
    List.concat_map discovered ~f:(fun (Path p) ->
        let idx = List.hd_exn p in
        let e0 = Array.get elevations idx in
        adjacent_positions idx height width
        |> List.filter ~f:(fun x -> if_move_valid e0 (Array.get elevations x))
        |> List.map ~f:(fun x -> Path (x :: p)))

  let update_paths nps sps =
    List.fold nps ~init:([], sps) ~f:(fun (acc_np, sps) np ->
        if is_shortest_path np sps then (np :: acc_np, add_path np sps)
        else (acc_np, add_path np sps))

  let discover_shortest_paths
      ({ map = { elevations; _ }; if_goal_reached; start_idx; _ } as rs) =
    let rec iter paths_to_goal paths shortest_paths =
      let new_paths = discover_paths rs paths in
      let new_paths_filtered, new_shortest_paths =
        update_paths new_paths shortest_paths
      in
      if List.is_empty new_paths_filtered then paths_to_goal
      else
        iter
          (List.filter new_paths_filtered ~f:(fun (Path x) ->
               if_goal_reached (Array.get elevations (List.hd_exn x)))
          @ paths_to_goal)
          new_paths_filtered new_shortest_paths
    in
    iter [] [ Path [ start_idx ] ] []

  let find_start_idx elevations f =
    fst (Array.findi_exn elevations ~f:(fun _ -> f))

  let discover_paths map if_move_valid if_goal_reached start_idx width height =
    discover_shortest_paths { map; if_move_valid; if_goal_reached; start_idx }
    |> List.map ~f:(fun (Path x) -> List.length x)
    |> List.fold ~init:(width * height) ~f:min
    |> fun x -> AnswerInt (x - 1)

  let solve_part1 (Input ({ width; height; elevations } as map)) =
    discover_paths map
      (function
        | End _ -> fun _ -> false
        | Start v1 | Elevation v1 -> (
            function Start _ -> false | End v2 | Elevation v2 -> v2 - v1 < 2))
      (function End _ -> true | _ -> false)
      (find_start_idx elevations (function Start _ -> true | _ -> false))
      width height

  let solve_part2 (Input ({ width; height; elevations } as map)) =
    discover_paths map
      (function
        | Elevation 0 | Start _ -> fun _ -> false
        | End v2 | Elevation v2 -> (
            function End _ -> false | Start v1 | Elevation v1 -> v2 - v1 < 2))
      (function Elevation 0 | Start _ -> true | _ -> false)
      (find_start_idx elevations (function End _ -> true | _ -> false))
      width height

  let part1 input_str =
    input_str |> parse_input |> solve_part1 |> answer_to_string

  let part2 input_str =
    input_str |> parse_input |> solve_part2 |> answer_to_string
end
