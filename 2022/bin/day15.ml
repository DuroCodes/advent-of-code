open Base
open Utils

module Day15 : Day = struct
  type range = XRange of int * int | YRange of int * int
  type direction = Horizontal | Vertical
  type position = int * int
  type sensor = { position : position; beacon : position }
  type input = Input of sensor list

  let parse_line l =
    let pattern =
      Str.regexp
        "Sensor at x=\\([0-9-]+\\), y=\\([0-9-]+\\): closest beacon is at \
         x=\\([0-9-]+\\), y=\\([0-9-]+\\)"
    in

    if Str.string_match pattern l 0 then
      let x1 = Str.matched_group 1 l |> Int.of_string in
      let y1 = Str.matched_group 2 l |> Int.of_string in
      let x2 = Str.matched_group 3 l |> Int.of_string in
      let y2 = Str.matched_group 4 l |> Int.of_string in
      { position = (x1, y1); beacon = (x2, y2) }
    else failwith "Unexpected input format"

  let parse_input t =
    t |> String.split_lines |> List.map ~f:parse_line |> fun x -> Input x

  let manhattan_distance (x1, y1) (x2, y2) =
    Int.abs (x1 - x2) + Int.abs (y1 - y2)

  let sensor_coverage direction coord { position; beacon } =
    let distance = manhattan_distance position beacon in
    let x, y = position in

    match direction with
    | Vertical ->
        let abs_dx = Int.abs (x - coord) in
        let yr = distance - abs_dx in
        if yr < 0 then None else Some (YRange (y - yr, y + yr))
    | Horizontal ->
        let abs_dy = Int.abs (y - coord) in
        let xr = distance - abs_dy in
        if xr < 0 then None else Some (XRange (x - xr, x + xr))

  let range_low_boundary = function
    | XRange (left, _) -> left
    | YRange (bottom, _) -> bottom

  let range_high_boundary = function
    | XRange (_, right) -> right
    | YRange (_, top) -> top

  let range_length = function
    | XRange (left, right) -> right - left + 1
    | YRange (bottom, top) -> top - bottom + 1

  let horizontal_boundaries y sensors =
    let xs =
      List.filter_map sensors ~f:(fun x -> sensor_coverage Horizontal y x)
    in
    let _ = YRange (0, 0) in
    let _ = Vertical in

    match
      ( List.min_elt (List.map xs ~f:range_low_boundary) ~compare:Int.compare,
        List.max_elt (List.map xs ~f:range_high_boundary) ~compare:Int.compare
      )
    with
    | Some left, Some right -> XRange (left, right)
    | _ -> failwith "Invalid boundaries"

  let solve_part1 (Input input) =
    let y = 2_000_000 in
    let beacons_on_y =
      input
      |> List.filter_map ~f:(fun { beacon = xb, yb; _ } ->
             if yb = y then Some xb else None)
      |> List.dedup_and_sort ~compare:Int.compare
      |> List.length
    in

    let sensor_coverage =
      List.filter_map input ~f:(fun x -> sensor_coverage Horizontal y x)
    in
    let global_boundaries = horizontal_boundaries y input in
    let global_low_x = range_low_boundary global_boundaries in
    let global_range_len = range_length global_boundaries in
    let coverage_arr = Array.init global_range_len ~f:(fun _ -> false) in

    let mark_local_range arr range =
      let range_low = range_low_boundary range in
      let range_len = range_length range in

      Array.init range_len ~f:Fn.id
      |> Array.iter ~f:(fun i ->
             Array.set arr (i + range_low - global_low_x) true);

      arr
    in

    sensor_coverage
    |> List.fold ~init:coverage_arr ~f:mark_local_range
    |> Array.count ~f:Fn.id
    |> fun x -> AnswerInt (x - beacons_on_y)

  let solve_part2 (Input input) =
    let min_x, max_x, max_y = (0, 4_000_000, 4_000_000) in
    let sensors_distances =
      List.map input ~f:(fun { position; beacon } ->
          (position, manhattan_distance position beacon))
    in
    let rec iter x y =
      if y > max_y then 0
      else if x > max_x then iter min_x (y + 1)
      else
        let diff =
          List.fold sensors_distances ~init:(-1) ~f:(fun z (position, dist) ->
              max z (dist - manhattan_distance position (x, y)))
        in
        if diff < 0 then (x * max_y) + y else iter (x + max diff 1) y
    in

    iter 0 0 |> fun x -> AnswerInt x

  let part1 input_str =
    input_str |> parse_input |> solve_part1 |> answer_to_string

  let part2 input_str =
    input_str |> parse_input |> solve_part2 |> answer_to_string
end
