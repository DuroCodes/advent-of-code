open Base
open Utils

module Day05 : Day = struct
  type proc = int * int * int
  type stack = char list
  type crane_model = CM9000 | CM9001
  type input = Input of { stacks : stack list; procs : proc list }

  let crate_from_stack line stack =
    let pos = (4 * stack) - 3 in
    if pos > String.length line then ' ' else String.get line pos

  let parse_stacks lines =
    let line_with_nums = List.hd_exn @@ List.drop (List.rev lines) 1 in
    let lines_with_crates = List.rev @@ List.drop (List.rev lines) 2 in
    let stack_nums =
      line_with_nums |> String.split ~on:' '
      |> List.filter_map ~f:(function
           | "" -> None
           | x -> Some (Int.of_string x))
    in
    stack_nums
    |> List.map ~f:(fun s ->
           List.map lines_with_crates ~f:(fun l -> crate_from_stack l s))
    |> List.map ~f:(fun s -> List.filter s ~f:(fun c -> Char.( <> ) c ' '))

  let parse_procs lines =
    let parse line =
      match String.split ~on:' ' line with
      | [ "move"; n; "from"; s; "to"; t ] ->
          Some (Int.of_string s, Int.of_string t, Int.of_string n)
      | _ -> None
    in
    List.filter_map lines ~f:parse

  let parse_input t =
    t |> String.split_lines |> List.group ~break:(fun x _ -> String.(x = ""))
    |> fun l ->
    let stacks = parse_stacks (List.hd_exn l) in
    let procs = parse_procs (List.hd_exn @@ List.drop l 1) in
    Input { stacks; procs }

  let rec restack first second n =
    match (first, second, n) with
    | [], _, _ | _, _, 0 -> (first, second)
    | hd :: tl, _, n -> restack tl (hd :: second) (n - 1)

  let restack_by_n first second n =
    let move, keep = List.split_n first n in
    (keep, List.append move second)

  let move_crates stacks proc model =
    let a = Array.of_list stacks in
    let first, second, num = proc in
    let f, t, n = (first - 1, second - 1, num) in
    let s1, s2 =
      match model with
      | CM9000 -> restack (Array.get a f) (Array.get a t) n
      | CM9001 -> restack_by_n (Array.get a f) (Array.get a t) n
    in
    Array.set a f s1;
    Array.set a t s2;
    Array.to_list a

  let solve_part1 (Input { stacks; procs }) =
    procs
    |> List.fold ~init:stacks ~f:(fun z x -> move_crates z x CM9000)
    |> List.filter_map ~f:List.hd
    |> fun x -> AnswerCharList x

  let solve_part2 (Input { stacks; procs }) =
    procs
    |> List.fold ~init:stacks ~f:(fun z x -> move_crates z x CM9001)
    |> List.filter_map ~f:List.hd
    |> fun x -> AnswerCharList x

  let part1 input_str =
    input_str |> parse_input |> solve_part1 |> answer_to_string

  let part2 input_str =
    input_str |> parse_input |> solve_part2 |> answer_to_string
end
