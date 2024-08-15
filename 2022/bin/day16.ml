open Core
open Utils

module Day16 : Day = struct
  type world = {
    relevant_rooms : int String.Map.t;
    valves : int String.Map.t;
    distances : int array array;
  }

  type room = { valve : string; flow_rate : int; leads_to : string list }
  type input = Input of world

  let floyd_warshall paths valves =
    let n = Map.length valves in
    let dist = Array.make_matrix ~dimx:n ~dimy:n 100 in
    for v = 0 to n - 1 do
      dist.(v).(v) <- 0
    done;
    let rec add_edges = function
      | [] -> ()
      | (src, dst) :: rest ->
          let u = Map.find_exn valves src and v = Map.find_exn valves dst in
          dist.(u).(v) <- 1;
          add_edges rest
    in
    add_edges paths;
    for k = 0 to n - 1 do
      for i = 0 to n - 1 do
        for j = 0 to n - 1 do
          let x = dist.(i).(k) + dist.(k).(j) in
          if dist.(i).(j) > x then dist.(i).(j) <- x
        done
      done
    done;
    dist

  let parse_line l =
    let pattern =
      Str.regexp
        "Valve \\([A-Z]+\\) has flow rate=\\([0-9]+\\); tunnels? leads? to \
         valves? \\(.*\\)"
    in
    if Str.string_match pattern l 0 then
      let valve = Str.matched_group 1 l in
      let flow_rate = Str.matched_group 2 l |> Int.of_string in
      let leads_to = Str.matched_group 3 l |> Str.split (Str.regexp ", ") in
      { valve; flow_rate; leads_to }
    else failwith "Unexpected input format"

  let parse_input t =
    let rooms = t |> String.split_lines |> List.map ~f:parse_line in
    let valves =
      List.mapi ~f:(fun index room -> (room.valve, index)) rooms
      |> String.Map.of_alist_exn
    in
    let paths =
      List.fold ~init:[]
        ~f:(fun acc room ->
          let pairs =
            List.map ~f:(fun tunnel -> (room.valve, tunnel)) room.leads_to
          in
          acc @ pairs)
        rooms
    in
    let relevant_rooms =
      List.filter rooms ~f:(fun room -> room.flow_rate > 0)
      |> List.map ~f:(fun room -> (room.valve, room.flow_rate))
      |> String.Map.of_alist_exn
    in
    let distances = floyd_warshall paths valves in

    Input { relevant_rooms; valves; distances }

  let distance world src dst =
    let u = Map.find_exn world.valves src in
    let v = Map.find_exn world.valves dst in
    world.distances.(u).(v)

  let create_adjacent_map world =
    let relevant_rooms = Map.keys world.relevant_rooms in
    let edges =
      List.fold relevant_rooms ~init:[] ~f:(fun acc src ->
          let others =
            List.filter relevant_rooms ~f:(fun dst ->
                String.compare src dst <> 0)
          in
          let pairs =
            List.fold others ~init:[] ~f:(fun pair_acc dst ->
                let dist = distance world src dst in
                (src, dst, dist) :: pair_acc)
          in
          acc @ pairs)
      @ List.fold relevant_rooms ~init:[] ~f:(fun acc dst ->
            let dist = distance world "AA" dst in
            ("AA", dst, dist) :: acc)
    in

    List.fold_left edges ~init:String.Map.empty
      ~f:(fun acc (src, dest, weight) ->
        Map.update acc src ~f:(function
          | Some neighbors -> (dest, weight) :: neighbors
          | None -> [ (dest, weight) ]))

  let find_paths world start_node max_time =
    let adjacent_map = create_adjacent_map world in

    let rec dfs node volume time_remaining path visited_nodes visited_paths =
      match Map.find adjacent_map node with
      | Some neighbors ->
          let candidates =
            List.filter neighbors ~f:(fun (node, weight) ->
                (not (Set.mem visited_nodes node))
                && time_remaining - weight - 1 > 0)
          in

          if List.is_empty candidates then
            visited_paths := (path, volume) :: !visited_paths
          else
            List.iter
              ~f:(fun (neighbor, weight) ->
                let flow_rate = Map.find_exn world.relevant_rooms neighbor in
                let new_time_remaining = time_remaining - weight - 1 in
                let new_volume = volume + (new_time_remaining * flow_rate) in
                let new_path = neighbor :: path in
                let new_visited_nodes = Set.add visited_nodes neighbor in
                dfs neighbor new_volume new_time_remaining new_path
                  new_visited_nodes visited_paths)
              candidates
      | None -> ()
    in
    let visited_paths = ref [] in
    let visited_nodes = String.Set.singleton start_node in
    dfs start_node 0 max_time [] visited_nodes visited_paths;
    !visited_paths

  let all_values_different a b =
    Set.inter (String.Set.of_list a) (String.Set.of_list b) |> Set.is_empty

  let solve_part1 (Input input) =
    find_paths input "AA" 30
    |> List.sort ~compare:(fun (_, a) (_, b) -> Int.compare b a)
    |> List.hd_exn |> snd
    |> fun x -> AnswerInt x

  let solve_part2 (Input input) =
    let candidates =
      find_paths input "AA" 26
      |> List.sort ~compare:(fun (p1, a) (p2, b) ->
             let cmp_vol = Int.compare b a in
             if cmp_vol <> 0 then cmp_vol else List.compare String.compare p1 p2)
    in

    let max = ref 0 in
    let rec check_candidates = function
      | [] -> ()
      | (elf, vol_a) :: rest ->
          List.iter rest ~f:(fun (elephant, vol_b) ->
              let total_flow = vol_a + vol_b in
              if total_flow > !max && all_values_different elf elephant then
                max := total_flow);
          check_candidates rest
    in

    check_candidates candidates;
    AnswerInt !max

  let part1 input_str =
    input_str |> parse_input |> solve_part1 |> answer_to_string

  let part2 input_str =
    input_str |> parse_input |> solve_part2 |> answer_to_string
end
