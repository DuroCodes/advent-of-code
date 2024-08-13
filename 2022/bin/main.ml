open Printf
open Utils

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
  let (module Day : Day) =
    match !day with
    | 1 -> (module Day01.Day01 : Day)
    | 2 -> (module Day02.Day02 : Day)
    | 3 -> (module Day03.Day03 : Day)
    | 4 -> (module Day04.Day04 : Day)
    | 5 -> (module Day05.Day05 : Day)
    | 6 -> (module Day06.Day06 : Day)
    | 7 -> (module Day07.Day07 : Day)
    | 8 -> (module Day08.Day08 : Day)
    | 9 -> (module Day09.Day09 : Day)
    | 10 -> (module Day10.Day10 : Day)
    | 11 -> (module Day11.Day11 : Day)
    | 12 -> (module Day12.Day12 : Day)
    | 13 -> (module Day13.Day13 : Day)
    | 14 -> (module Day14.Day14 : Day)
    | 15 -> (module Day15.Day15 : Day)
    | _ -> raise (InvalidDay !day)
  in
  let input_str =
    if String.equal !input_file "" then In_channel.input_all Stdio.stdin
    else read_input_file !input_file
  in
  let answer1 = Day.part1 input_str in
  let answer2 = Day.part2 input_str in
  printf "Part 1: %s\n" answer1;
  printf "Part 2: %s\n" answer2
