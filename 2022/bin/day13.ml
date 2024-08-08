open Base
open Utils

module Day13 : Day = struct
  type packet =
    | PacketInt of int * packet
    | PacketList of packet * packet
    | PacketNull

  type pair = packet * packet
  type input = Input of pair list

  let parse_line l =
    let rec read_number = function
      | [] -> ([], [])
      | c :: cs ->
          if Char.( >= ) c '0' && Char.( <= ) c '9' then
            let a, b = read_number cs in
            (c :: a, b)
          else ([], c :: cs)
    in
    let read_group chars =
      let rec scan lev chars =
        match (lev, chars) with
        | 0, '[' :: cs ->
            let a, b = scan 1 cs in
            (a, b)
        | 1, ']' :: cs -> ([], cs)
        | l, '[' :: cs ->
            let a, b = scan (l + 1) cs in
            ('[' :: a, b)
        | l, ']' :: cs ->
            let a, b = scan (l - 1) cs in
            (']' :: a, b)
        | l, c :: cs ->
            let a, b = scan l cs in
            (c :: a, b)
        | _ -> failwith "Invalid Group"
      in

      scan 0 chars
    in

    let rec iter = function
      | [] -> PacketNull
      | ']' :: [] -> PacketNull
      | ',' :: rem -> iter rem
      | '[' :: ']' :: rem -> PacketList (PacketNull, iter rem)
      | '[' :: rem ->
          let g, rem = read_group ('[' :: rem) in
          PacketList (iter g, iter rem)
      | rem ->
          let num, rem = read_number rem in
          let num = Int.of_string @@ String.of_char_list num in
          PacketInt (num, iter rem)
    in

    iter (String.to_list l)

  let parse_input t =
    t |> String.split_lines
    |> List.group ~break:(fun x _ -> String.equal x "")
    |> List.map ~f:(fun x ->
           (parse_line (List.hd_exn x), parse_line (List.nth_exn x 1)))
    |> fun x -> Input x

  let rec compare_packets = function
    | PacketInt (l, ls), PacketInt (r, rs) ->
        if l <> r then Int.compare l r else compare_packets (ls, rs)
    | PacketList (l, ls), PacketList (r, rs) ->
        let res = compare_packets (l, r) in
        if res = 0 then compare_packets (ls, rs) else res
    | PacketNull, (PacketInt _ | PacketList _) -> -1
    | (PacketInt _ | PacketList _), PacketNull -> 1
    | PacketInt (l, ls), PacketList (r, rs) ->
        let res = compare_packets (PacketInt (l, PacketNull), r) in
        if res = 0 then compare_packets (ls, rs) else res
    | PacketList (l, ls), PacketInt (r, rs) ->
        let res = compare_packets (l, PacketInt (r, PacketNull)) in
        if res = 0 then compare_packets (ls, rs) else res
    | PacketNull, PacketNull -> 0

  let solve_part1 (Input input) =
    input
    |> List.map ~f:compare_packets
    |> List.filter_mapi ~f:(fun i x -> if x = -1 then Some (i + 1) else None)
    |> List.fold ~init:0 ~f:( + )
    |> fun x -> AnswerInt x

  let solve_part2 (Input input) =
    let dp2 =
      PacketList (PacketList (PacketInt (2, PacketNull), PacketNull), PacketNull)
    in
    let dp6 =
      PacketList (PacketList (PacketInt (6, PacketNull), PacketNull), PacketNull)
    in

    (dp2, dp6) :: input
    |> List.concat_map ~f:(fun (left, right) -> [ left; right ])
    |> List.sort ~compare:(fun left right -> compare_packets (left, right))
    |> List.filter_mapi ~f:(fun i x ->
           if compare_packets (x, dp2) = 0 || compare_packets (x, dp6) = 0 then
             Some (i + 1)
           else None)
    |> List.fold ~init:1 ~f:( * )
    |> fun x -> AnswerInt x

  let part1 input_str =
    input_str |> parse_input |> solve_part1 |> answer_to_string

  let part2 input_str =
    input_str |> parse_input |> solve_part2 |> answer_to_string
end
