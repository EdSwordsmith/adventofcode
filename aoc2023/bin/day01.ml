let int_of_char_opt c = String.make 1 c |> int_of_string_opt

let parse_line line =
  let helper (f, l) c = match int_of_char_opt c with
    | None -> (f, l)
    | Some v -> match f, l with
      | None, _ -> (Some v, Some v)
      | Some _, _ -> (f, Some v) in
  let (f, l) = String.fold_left helper (None, None) line in
  Option.get f * 10 + Option.get l

let parse_line' line =
  let parse_written s = match s with
        | 'o'::'n'::'e'::_ -> Some 1
        | 't'::'w'::'o'::_ -> Some 2
        | 't'::'h'::'r'::'e'::'e'::_ -> Some 3
        | 'f'::'o'::'u'::'r'::_ -> Some 4
        | 'f'::'i'::'v'::'e'::_ -> Some 5
        | 's'::'i'::'x'::_ -> Some 6
        | 's'::'e'::'v'::'e'::'n'::_ -> Some 7
        | 'e'::'i'::'g'::'h'::'t'::_ -> Some 8
        | 'n'::'i'::'n'::'e'::_ -> Some 9
        | _ -> None in

  let parse_digit s = match s with
    | [] -> None, []
    | h::tl -> match int_of_char_opt h with
      | Some v -> Some v, tl
      | None -> parse_written s, tl in

  let rec parse_digits s acc = match s with
    | [] -> acc
    | _ -> let d, s' = parse_digit s in
      match acc, d with
      | _, None -> parse_digits s' acc
      | (None, _), Some _ -> parse_digits s' (d, d)
      | (Some v, _), Some _ -> parse_digits s' (Some v, d) in

  let chars = List.init (String.length line) (String.get line) in
  let f, l = parse_digits chars (None, None) in

  Option.get f * 10 + Option.get l

let solve () =
  let maybe_read_line () = try Some (read_line ()) with End_of_file -> None in
  let rec solve_helper (acc, acc') = match maybe_read_line () with
    | None -> (acc, acc')
    | Some line -> let v, v' = parse_line line, parse_line' line in
      solve_helper (v + acc, v' + acc') in
  solve_helper (0, 0)

let () = let p1, p2 = solve () in
  Printf.printf "Part 1: %d\n" p1;
  Printf.printf "Part 2: %d\n" p2
