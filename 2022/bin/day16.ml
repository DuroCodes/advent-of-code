open Core
open Utils

module Day16 : Day = struct
  type tunnel = (string * int) list
  type valve = string * int * (string * int) list

  type map = {
    mutable tunnels : (string, tunnel) Hashtbl.t;
    rates : (string, int) Hashtbl.t;
    mutable rate_sum : int;
  }

  type input = Input of valve list

  let parse_line l =
    let matches =
      let pattern =
        "^Valve ([A-Z]+) has flow rate=([0-9]+); tunnels* leads* to valves* \
         ([A-Z, ]+)$"
      in
      Re.exec (Re.Posix.compile_pat pattern) l
    in

    ( Re.Group.get matches 1,
      Int.of_string @@ Re.Group.get matches 2,
      Re.Group.get matches 3
      |> String.split_on_chars ~on:[ ' '; ',' ]
      |> List.filter ~f:(Fn.non String.is_empty)
      |> List.map ~f:(fun t -> (t, 1)) )

  let parse_input t =
    t |> String.split_lines |> List.map ~f:parse_line |> fun x -> Input x

  let bfs map start =
    let queue = Queue.create () in
    let distances = Hashtbl.Poly.create () in
    Queue.enqueue queue (0, start);
    let rec loop () =
      if Queue.length queue = 0 then ()
      else
        let distance, valve = Queue.dequeue_exn queue in
        Hashtbl.find_exn map.tunnels valve
        |> List.map ~f:(fun (a, _) -> a)
        |> List.filter ~f:(fun n -> not (Hashtbl.mem distances n))
        |> List.iter ~f:(fun n ->
               Hashtbl.set distances ~key:n ~data:(distance + 1);
               Queue.enqueue queue (distance + 1, n));
        loop ()
    in
    loop ();
    distances

  let enrich map =
    let new_tunnels = Hashtbl.Poly.create () in
    let important =
      "AA"
      :: (Hashtbl.keys map.tunnels
         |> List.filter ~f:(fun t -> Hashtbl.find_exn map.rates t > 0))
    in

    let update_tunnels n =
      let distances = bfs map n in
      let tunnels =
        important
        |> List.filter ~f:(fun n' ->
               String.( <> ) n' "AA" && String.( <> ) n n')
        |> List.map ~f:(fun n' -> (n', Hashtbl.find_exn distances n'))
      in
      Hashtbl.set new_tunnels ~key:n ~data:tunnels
    in

    List.iter important ~f:update_tunnels;
    map.tunnels <- new_tunnels;
    map

  let add_tunnels map (from, rate, to') =
    Hashtbl.set map.tunnels ~key:from ~data:to';
    Hashtbl.set map.rates ~key:from ~data:rate;
    map.rate_sum <- map.rate_sum + rate;
    map

  let find map =
    let max_rates = ref 0 in
    let rec find' (rate, time, used_rate, path) =
      if rate + ((map.rate_sum - used_rate) * (time - 2)) < !max_rates then rate
      else (
        max_rates := max !max_rates rate;
        let current = List.hd_exn path in
        let neighbors = Hashtbl.find_exn map.tunnels current in
        if time <= 0 then rate
        else
          let next (gate, distance) =
            let gate_rate = Hashtbl.find_exn map.rates gate in
            let new_rate =
              rate
              + (time - 1 - distance)
                *
                if List.mem path gate ~equal:String.equal then 0 else gate_rate
            in
            find'
              ( new_rate,
                time - 1 - distance,
                used_rate + gate_rate,
                gate :: path )
          in
          List.map ~f:next neighbors |> List.fold ~init:0 ~f:max)
    in
    find' (0, 30, 0, [ "AA" ])

  let find2 map =
    let max_rates = ref 0 in
    let rec find'
        ( rate,
          (time, elephant_time),
          used_rate,
          (current, current_elephant),
          path ) =
      if
        rate + ((map.rate_sum - used_rate) * (min time elephant_time - 2))
        < !max_rates
      then rate
      else (
        max_rates := max !max_rates rate;
        let neighbors = Hashtbl.find_exn map.tunnels current in
        let elephant_neighbors =
          Hashtbl.find_exn map.tunnels current_elephant
        in
        if time <= 0 || elephant_time <= 0 then rate
        else
          let next next_time is_elephant (gate, distance) =
            let gate_rate = Hashtbl.find_exn map.rates gate in
            let new_rate =
              rate
              + (next_time - 1 - distance)
                *
                if List.mem path gate ~equal:String.equal then 0 else gate_rate
            in
            if is_elephant then
              [
                find'
                  ( new_rate,
                    (time, next_time - 1 - distance),
                    used_rate + gate_rate,
                    (current, gate),
                    gate :: path );
              ]
            else
              [
                find'
                  ( new_rate,
                    (next_time - 1 - distance, elephant_time),
                    used_rate + gate_rate,
                    (gate, current_elephant),
                    gate :: path );
              ]
          in
          List.concat
            [
              List.concat_map ~f:(next time false) neighbors;
              List.concat_map ~f:(next elephant_time true) elephant_neighbors;
            ]
          |> List.fold ~init:0 ~f:max)
    in
    find' (0, (26, 26), 0, ("AA", "AA"), [ "AA" ])

  let enrich_input input =
    let map =
      {
        tunnels = Hashtbl.Poly.create ();
        rates = Hashtbl.Poly.create ();
        rate_sum = 0;
      }
    in
    input |> List.fold ~init:map ~f:add_tunnels |> enrich

  let solve_part1 (Input input) =
    enrich_input input |> find |> fun x -> AnswerInt x

  let solve_part2 (Input input) =
    enrich_input input |> find2 |> fun x -> AnswerInt x

  let part1 input_str =
    input_str |> parse_input |> solve_part1 |> answer_to_string

  let part2 input_str =
    input_str |> parse_input |> solve_part2 |> answer_to_string
end
