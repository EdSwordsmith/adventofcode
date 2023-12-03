module PosMap = Map.Make (struct
  type t = int * int
  let compare = compare
end)

module PNSet = Set.Make (struct
  type t = int * int * int * int
  let compare = compare
end)

let merge_entries _ e1 e2 =
  match (e1, e2) with _, None -> e1 | None, _ -> e2 | _, _ -> None

let rec range i j = if j < i then [] else i :: range (i + 1) j

let range2d (col, col') (row, row') =
  let cols, rows = (range col col', range row row') in
  List.flatten
    (List.map (fun row -> List.map (fun col -> (row, col)) cols) rows)

let parse_line line row =
  let is_digit c =
    let v = int_of_char c in
    v >= 48 && v <= 57
  in

  let rec ignore_dots chars index =
    match chars with
    | [] -> (chars, index)
    | '.' :: tl -> ignore_dots tl (index + 1)
    | _ -> (chars, index)
  in

  let rec parse_num chars index acc =
    match chars with
    | [] -> (chars, index, acc)
    | h :: tl ->
        let v = int_of_char h - 48 in
        if is_digit h then parse_num tl (index + 1) ((acc * 10) + v)
        else (chars, index, acc)
  in

  let rec helper chars index numbers symbols =
    match chars with
    | [] -> (numbers, symbols)
    | '.' :: _ ->
        let chars, index = ignore_dots chars index in
        helper chars index numbers symbols
    | h :: tl ->
        if is_digit h then
          let chars, index', num = parse_num chars index 0 in
          let cols = range index (index' - 1) in
          let entries =
            List.map
              (fun col -> ((row, col), (num, row, index, index' - 1)))
              cols
          in
          let nums = entries |> List.to_seq |> PosMap.of_seq in
          let numbers = PosMap.merge merge_entries numbers nums in
          helper chars index' numbers symbols
        else helper tl (index + 1) numbers (PosMap.add (row, index) h symbols)
  in

  let chars = List.init (String.length line) (String.get line) in
  helper chars 0 PosMap.empty PosMap.empty

let parse_input () =
  let maybe_read_line () = try Some (read_line ()) with End_of_file -> None in
  let rec helper numbers symbols row =
    match maybe_read_line () with
    | None -> (numbers, symbols)
    | Some line ->
        let nums, syms = parse_line line row in
        let numbers = PosMap.merge merge_entries numbers nums in
        let symbols = PosMap.merge merge_entries symbols syms in
        helper numbers symbols (row + 1)
  in
  helper PosMap.empty PosMap.empty 0

let solve_p1 numbers symbols =
  let has_adj_syms (_, r, s, e) =
    let positions = range2d (s - 1, e + 1) (r - 1, r + 1) in
    let syms = List.filter (fun p -> PosMap.mem p symbols) positions in
    List.length syms > 0
  in

  PosMap.to_seq numbers
  |> Seq.map (fun (_, number) -> number)
  |> Seq.filter has_adj_syms
  |> Seq.map (fun (n, _, _, _) -> n)
  |> Seq.fold_left ( + ) 0

let solve_p2 numbers symbols =
  let get_adj_nums (r, c) =
    let positions = range2d (c - 1, c + 1) (r - 1, r + 1) in
    let nums = List.map (fun p -> PosMap.find_opt p numbers) positions in
    let nums = List.map Option.to_list nums |> List.flatten in
    List.fold_right PNSet.add nums PNSet.empty
  in

  let stars =
    PosMap.filter (fun _ s -> s = '*') symbols
    |> PosMap.to_seq
    |> Seq.map (fun (p, _) -> p)
  in
  let gears =
    Seq.map get_adj_nums stars |> Seq.filter (fun pns -> PNSet.cardinal pns = 2)
  in
  let ratios =
    Seq.map
      (fun pns -> PNSet.fold (fun (n, _, _, _) acc -> n * acc) pns 1)
      gears
  in
  Seq.fold_left ( + ) 0 ratios

let () =
  let numbers, symbols = parse_input () in
  let p1, p2 = (solve_p1 numbers symbols, solve_p2 numbers symbols) in
  Printf.printf "Part 1: %d\n" p1;
  Printf.printf "Part 2: %d\n" p2
