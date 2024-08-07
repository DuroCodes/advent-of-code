open Base
open Utils

module Day11 : Day = struct
  type monkey = {
    id : int;
    items : int list;
    operation : int -> int;
    test : int -> bool;
    divisible_by : int;
    if_true : int;
    if_false : int;
    inspected : int;
  }

  type thrown_item = { target : int; item : int }
  type input = Input of monkey list

  let parse_monkey input =
    let parse_operation v1 op v2 x =
      let f =
        match op with
        | "+" -> ( + )
        | "*" -> ( * )
        | _ -> failwith "Invalid Operation"
      in

      f
        (match v1 with "old" -> x | _ -> Int.of_string v1)
        (match v2 with "old" -> x | _ -> Int.of_string v2)
    in

    let extract prefix s = String.chop_prefix_exn ~prefix s in
    let parse_items s =
      String.split_on_chars ~on:[ ','; ' ' ] s
      |> List.filter_map ~f:(function
           | "" -> None
           | x -> Some (Int.of_string x))
    in

    let id =
      extract "Monkey " (List.nth_exn input 0)
      |> String.chop_suffix_exn ~suffix:":"
      |> Int.of_string
    in
    let items =
      extract "  Starting items: " (List.nth_exn input 1) |> parse_items
    in
    let op_components =
      extract "  Operation: new = " (List.nth_exn input 2)
      |> String.split_on_chars ~on:[ ' ' ]
    in
    let operation =
      parse_operation
        (List.nth_exn op_components 0)
        (List.nth_exn op_components 1)
        (List.nth_exn op_components 2)
    in
    let divisible_by =
      extract "  Test: divisible by " (List.nth_exn input 3) |> Int.of_string
    in
    let test v = v % divisible_by = 0 in
    let if_true =
      extract "    If true: throw to monkey " (List.nth_exn input 4)
      |> Int.of_string
    in
    let if_false =
      extract "    If false: throw to monkey " (List.nth_exn input 5)
      |> Int.of_string
    in
    {
      id;
      items;
      operation;
      divisible_by;
      test;
      if_true;
      if_false;
      inspected = 0;
    }

  let parse_input t =
    t |> String.split_lines
    |> List.group ~break:(fun x _ -> String.is_empty x)
    |> List.map ~f:parse_monkey
    |> fun x -> Input x

  let throw_items monkey relief =
    monkey.items
    |> List.map ~f:monkey.operation
    |> List.map ~f:(fun x -> x / relief)
    |> List.map ~f:(fun item ->
           match monkey.test item with
           | true -> { target = monkey.if_true; item }
           | false -> { target = monkey.if_false; item })

  let redistribute_items current monkeys items =
    let divisor =
      List.fold monkeys ~init:1 ~f:(fun acc m -> acc * m.divisible_by)
    in

    let process_monkey m =
      let receiving =
        List.filter_map items ~f:(fun { target; item } ->
            if target = m.id then Some item else None)
      in
      if m.id = current.id then
        { m with items = []; inspected = m.inspected + List.length m.items }
      else { m with items = List.append m.items receiving }
    in

    monkeys |> List.map ~f:process_monkey
    |> List.map ~f:(fun m ->
           { m with items = List.map m.items ~f:(fun x -> x % divisor) })

  let play_round monkeys relief =
    let rec play_turn acc = function
      | [] -> acc
      | turn :: ts ->
          let m = List.nth_exn acc turn in
          let thrown = throw_items m relief in
          let new_acc = redistribute_items m acc thrown in
          play_turn new_acc ts
    in
    play_turn monkeys (List.init (List.length monkeys) ~f:Fn.id)

  let solve_part1 (Input input) =
    List.init 20 ~f:Fn.id
    |> List.fold ~init:input ~f:(fun z _ -> play_round z 3)
    |> List.map ~f:(fun m -> m.inspected)
    |> List.sort ~compare:Int.compare
    |> (fun x -> List.take (List.rev x) 2)
    |> List.fold ~init:1 ~f:( * )
    |> fun x -> AnswerInt x

  let solve_part2 (Input input) =
    List.init 10_000 ~f:Fn.id
    |> List.fold ~init:input ~f:(fun z _ -> play_round z 1)
    |> List.map ~f:(fun m -> m.inspected)
    |> List.sort ~compare:Int.compare
    |> (fun x -> List.take (List.rev x) 2)
    |> List.fold ~init:1 ~f:( * )
    |> fun x -> AnswerInt x

  let part1 input_str =
    input_str |> parse_input |> solve_part1 |> answer_to_string

  let part2 input_str =
    input_str |> parse_input |> solve_part2 |> answer_to_string
end
