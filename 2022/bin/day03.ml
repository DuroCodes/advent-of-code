open Core
open Utils

module Day03 : Day = struct
  type item = char
  type compartment = item list
  type sack = compartment * compartment
  type input = Input of sack list

  let parse_input t =
    t |> String.split_lines |> List.map ~f:String.to_list
    |> List.map ~f:(fun x -> List.split_n x (List.length x / 2))
    |> fun x -> Input x

  let item_in_both (comp1, comp2) =
    let common = List.filter comp1 ~f:(fun x -> List.exists comp2 ~f:(Char.equal x)) in
    match common with
    | [] -> '0'
    | hd :: _ -> hd

  let item_priority item =
    let priority ch =
      if Char.is_uppercase ch then Char.to_int ch - Char.to_int 'A' + 27
      else Char.to_int ch - Char.to_int 'a' + 1
    in
    priority item

  let item_in_sacks sacks =
    let items = List.map sacks ~f:(fun (comp1, comp2) -> List.append comp1 comp2) in
    let hd, tl = (List.hd_exn items, List.tl_exn items) in
    let common first second =
      List.filter first ~f:(fun x -> List.exists second ~f:(Char.equal x))
    in
    let common_items = List.fold tl ~init:hd ~f:common in
    match common_items with
    | [] -> '0'
    | hd :: _ -> hd

  let solve_part1 (Input input) =
    input |> List.map ~f:item_in_both |> List.map ~f:item_priority |> List.fold ~init:0 ~f:( + )
    |> fun x -> AnswerInt x

  let solve_part2 (Input input) =
    input |> List.chunks_of ~length:3 |> List.map ~f:item_in_sacks |> List.map ~f:item_priority
    |> List.fold ~init:0 ~f:( + )
    |> fun x -> AnswerInt x

  let part1 input_str = input_str |> parse_input |> solve_part1 |> answer_to_string
  let part2 input_str = input_str |> parse_input |> solve_part2 |> answer_to_string
end
