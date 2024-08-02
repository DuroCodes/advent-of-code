let day = ref (-1)
let input_file = ref ""
let anon_fun filename = input_file := filename

let speclist =
  [
    ("-d", Arg.Set_int day, "Day to run");
    ("-i", Arg.String anon_fun, "Input file");
  ]

exception InvalidDay of int

let read_input_file filename =
  let ch = open_in filename in
  let s = really_input_string ch (in_channel_length ch) in
  close_in ch;
  s

let () =
  let () =
    Arg.parse speclist anon_fun "Usage: advent-of-code -d <day> -i <input file>"
  in
  let part1, part2 =
    match !day with
    | 1 -> (Day01.part1, Day01.part2)
    | 2 -> (Day02.part1, Day02.part2)
    | _ -> raise (InvalidDay !day)
  in
  let open Stdio in
  let input_str =
    if String.equal !input_file "" then In_channel.input_all Stdio.stdin
    else read_input_file !input_file
  in
  let answer1 = part1 input_str in
  let answer2 = part2 input_str in
  printf "Part 1: %s\n" answer1;
  printf "Part 2: %s\n" answer2
