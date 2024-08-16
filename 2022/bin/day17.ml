open Core
open Utils

module Day17 : Day = struct
  type rock = Minus | Plus | Corner | Pipe | Square
  type direction = Left | Right | Down
  type map = { mutable rocks : (int * int) Hash_set.t; mutable highest : int }
  type moves = { move_items : direction array; mutable next_move : int }
  type rocks = { rock_items : rock list; mutable next_rock : int }
  type input = Input of moves

  let parse_char = function
    | '<' -> Left
    | '>' -> Right
    | _ -> failwith "Invalid direction"

  let parse_input t =
    let move_items = Array.of_list @@ List.map (String.to_list t) ~f:parse_char in
    Input { move_items; next_move = 0 }

  let create_rock (x, y) =
    let rock positions = Hash_set.Poly.of_list positions in

    function
    | Minus -> rock [ (x + 3, y); (x + 4, y); (x + 5, y); (x + 6, y) ]
    | Plus -> rock [ (x + 4, y); (x + 3, y + 1); (x + 4, y + 1); (x + 5, y + 1); (x + 4, y + 2) ]
    | Corner -> rock [ (x + 3, y); (x + 4, y); (x + 5, y); (x + 5, y + 1); (x + 5, y + 2) ]
    | Pipe -> rock [ (x + 3, y); (x + 3, y + 1); (x + 3, y + 2); (x + 3, y + 3) ]
    | Square -> rock [ (x + 3, y); (x + 4, y); (x + 3, y + 1); (x + 4, y + 1) ]

  let get_next_move moves =
    moves.next_move <- (moves.next_move mod Array.length moves.move_items) + 1;
    Array.get moves.move_items (moves.next_move - 1)

  let get_next_rock rocks =
    rocks.next_rock <- (rocks.next_rock mod List.length rocks.rock_items) + 1;
    List.nth_exn rocks.rock_items (rocks.next_rock - 1)

  let move (x, y) = function
    | Left -> (x - 1, y)
    | Right -> (x + 1, y)
    | Down -> (x, y - 1)

  let move_rock rock direction =
    let move' pos = move pos direction in
    Hash_set.Poly.of_list @@ List.map (Hash_set.to_list rock) ~f:move'

  let collides rock map =
    let min_max f rock =
      ( Option.value_exn (Hash_set.min_elt ~compare:f rock),
        Option.value_exn (Hash_set.max_elt ~compare:f rock) )
    in

    let left, right = min_max (fun (x1, _) (x2, _) -> Int.compare x1 x2) rock in
    let bottom, _ = min_max (fun (_, y1) (_, y2) -> Int.compare y1 y2) rock in

    fst left <= 0
    || fst right >= 8
    || snd bottom <= 0
    || Hash_set.length (Hash_set.inter rock map.rocks) > 0

  let add_rock rock map =
    map.rocks <- Hash_set.union rock map.rocks;
    Hash_set.filter_inplace map.rocks ~f:(fun (_, y) -> y > map.highest - 50)

  let rec place_rock rock map moves =
    let next_move = get_next_move moves in
    let next_rock = move_rock rock next_move in
    let next_rock = if collides next_rock map then rock else next_rock in
    let next_rock' = move_rock next_rock Down in

    if collides next_rock' map then (
      add_rock next_rock map;
      let highest =
        Option.value_exn
          (Hash_set.max_elt ~compare:(fun (_, y1) (_, y2) -> Int.compare y1 y2) map.rocks)
      in
      map.highest <- snd highest)
    else place_rock next_rock' map moves

  let solve_part1 (Input input) =
    let figures = { rock_items = [ Minus; Plus; Corner; Pipe; Square ]; next_rock = 0 } in
    let map = { rocks = Hash_set.Poly.create (); highest = 0 } in

    for _ = 1 to 2022 do
      let rock = create_rock (0, map.highest + 4) (get_next_rock figures) in
      place_rock rock map input
    done;

    AnswerInt map.highest

  let solve_part2 (Input input) =
    let figures = { rock_items = [ Minus; Plus; Corner; Pipe; Square ]; next_rock = 0 } in
    let map = { rocks = Hash_set.Poly.create (); highest = 0 } in

    for _ = 1 to 1_000_000_000_000 do
      let next_rock = get_next_rock figures in
      let rock = create_rock (0, map.highest + 4) next_rock in
      place_rock rock map input
    done;

    AnswerInt map.highest

  let part1 input_str = input_str |> parse_input |> solve_part1 |> answer_to_string
  let part2 input_str = input_str |> parse_input |> solve_part2 |> answer_to_string
end
