open Core
open Utils

module Day16 : Day = struct
  type world = {
    relevant_rooms : int String.Map.t;
    valves : int String.Map.t;
    distances : int array array;
  }

  type valve = { name : string; flow_rate : int; leads_to : string list }
  type input = Input of valve list

  let parse_line l =
    let pattern =
      Str.regexp
        "Valve (.*) has flow rate=(\\d+); tunnels? leads? to valves? (.*)"
    in
    if Str.string_match pattern l 0 then
      let name = Str.matched_group 1 l in
      let flow_rate = Str.matched_group 2 l |> Int.of_string in
      let leads_to = Str.matched_group 3 l |> Str.split (Str.regexp ", ") in
      { name; flow_rate; leads_to }
    else failwith "Unexpected input format"

  let parse_input t =
    t |> String.split_lines |> List.map ~f:parse_line |> fun x -> Input x

  let floyd_warshall paths valves =
    let n = Map.length valves in
    let dist = Array.make_matrix ~dimx:n ~dimy:n Int.max_value in
    let dist =
      Array.init n ~f:(fun i ->
          Array.init n ~f:(fun j -> if i = j then 0 else dist.(i).(j)))
    in

    let rec add_edges = function
      | [] -> ()
      | (src, dst) :: rest ->
          let u = Map.find_exn valves src in
          let v = Map.find_exn valves dst in

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

  let solve_part1 (Input _) = Unknown
  let solve_part2 (Input _) = Unknown

  let part1 input_str =
    input_str |> parse_input |> solve_part1 |> answer_to_string

  let part2 input_str =
    input_str |> parse_input |> solve_part2 |> answer_to_string
end
