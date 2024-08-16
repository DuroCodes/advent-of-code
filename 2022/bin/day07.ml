open Core
open Utils

module Day07 : Day = struct
  type item =
    | Directory of { name : string; size : int option; content : item list }
    | File of { name : string; size : int }

  type input = Input of item
  type line = Cd of string | Ls | Dir of string | Output of int * string

  let parse_line line =
    match String.split ~on:' ' line with
    | "$" :: "cd" :: dir :: _ -> Cd dir
    | "$" :: "ls" :: _ -> Ls
    | "dir" :: dir :: _ -> Dir dir
    | size :: name :: _ -> Output (Int.of_string size, name)
    | _ -> failwith "Invalid input"

  let cd path dir =
    match dir with
    | "/" -> []
    | "." -> path
    | ".." -> List.take path (List.length path - 1)
    | _ -> List.append path [ dir ]

  let rec add_item tree path item =
    match tree with
    | File { name = _; size = _ } -> failwith "Cannot add item to a file"
    | Directory { name; size; content } -> (
        match path with
        | [] -> Directory { name; size; content = item :: content }
        | p :: ps -> (
            let filter = function
              | Directory { name; _ } -> String.(name = p)
              | _ -> false
            in
            match List.partition_tf content ~f:filter with
            | [ sub_dir ], other_dirs ->
                Directory { name; size; content = add_item sub_dir ps item :: other_dirs }
            | _ -> failwith "Invalid path"))

  let fold_lines (path, tree) line =
    match parse_line line with
    | Ls -> (path, tree)
    | Cd dir -> (cd path dir, tree)
    | Dir dir ->
        let new_dir = Directory { name = dir; size = None; content = [] } in
        (path, add_item tree path new_dir)
    | Output (size, name) ->
        let new_file = File { name; size } in
        (path, add_item tree path new_file)

  let parse_input t =
    t |> String.split_lines
    |> List.fold ~init:([], Directory { name = ""; size = None; content = [] }) ~f:fold_lines
    |> snd
    |> fun tree -> Input tree

  let rec dir_size tree =
    match tree with
    | File { size; _ } -> size
    | Directory { size = Some size; _ } -> size
    | Directory { content; _ } -> List.fold content ~init:0 ~f:(fun acc item -> acc + dir_size item)

  let rec update_dir_sizes tree =
    match tree with
    | File _ as file -> file
    | Directory ({ content; _ } as dir) ->
        let updated_content = List.map content ~f:update_dir_sizes in
        let size = List.fold updated_content ~init:0 ~f:(fun acc item -> acc + dir_size item) in
        Directory { dir with size = Some size; content = updated_content }

  let rec get_dirs = function
    | [] -> []
    | Directory { name; size = Some size; content } :: tl ->
        (name, size) :: (get_dirs content @ get_dirs tl)
    | _ :: tl -> get_dirs tl

  let solve_part1 (Input input) =
    get_dirs [ update_dir_sizes input ]
    |> List.filter_map ~f:(fun (_, size) -> if size > 100_000 then None else Some size)
    |> List.fold ~init:0 ~f:( + )
    |> fun x -> AnswerInt x

  let solve_part2 (Input input) =
    let total_space = 70_000_000 in
    let required_space = 30_000_000 in
    let used_space = dir_size input in
    let needed_space = required_space - (total_space - used_space) in
    update_dir_sizes input |> fun tree ->
    get_dirs [ tree ]
    |> List.filter_map ~f:(fun (_, size) -> if size >= needed_space then Some size else None)
    |> List.min_elt ~compare:Int.compare
    |> function
    | Some x -> AnswerInt x
    | None -> AnswerInt 0

  let part1 input_str = input_str |> parse_input |> solve_part1 |> answer_to_string
  let part2 input_str = input_str |> parse_input |> solve_part2 |> answer_to_string
end
