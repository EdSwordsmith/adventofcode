let parse_game line =
  let ( *> ) p1 p2 chars = match p1 chars with
    | Some _, chars' -> p2 chars'
    | None, _ -> None, chars in

  let ( <* ) p1 p2 chars = match p1 chars with
    | None, _ -> None, chars
    | Some v, chars' -> match p2 chars' with
      | None, _ -> None, chars
      | Some _, chars' -> Some v, chars' in

  let parse_char char chars = match chars with
    | [] -> None, []
    | h :: tl -> if h = char then Some h, tl else None, chars in

  let parse_digit chars = match chars with
    | [] -> None, []
    | h :: tl -> let ascii_code = int_of_char h in
      if ascii_code >= 48 && ascii_code <= 57 then Some (ascii_code - 48), tl else None, chars in

  let parse_number chars =
    let rec helper chars acc = match parse_digit chars, acc with
        | (None, chars'), _ -> acc, chars'
        | (Some d, chars'), None -> helper chars' (Some d)
        | (Some d, chars'), Some v -> helper chars' (Some (v * 10 + d)) in
    helper chars None in

  let parse_start chars =
    let num, chars' = match chars with
        | 'G' :: 'a' :: 'm' :: 'e' :: ' ' :: chars' -> parse_number chars'
        | _ -> None, chars in
    let _, chars' = parse_char ':' chars' in
        num, chars' in

  let parse_color chars =
    let parse_color_name chars = match chars with
      | 'r' :: 'e' :: 'd' :: tl -> Some (1, 0, 0), tl
      | 'g' :: 'r' :: 'e' :: 'e' :: 'n' :: tl -> Some (0, 1, 0), tl
      | 'b' :: 'l' :: 'u' :: 'e' :: tl -> Some (0, 0, 1), tl
      | _ -> None, chars in

    let q, chars' = (parse_char ' ' *> parse_number <* parse_char ' ') chars in
    let color, chars' = parse_color_name chars' in
    match q, color with
    | None, _ -> None, chars
    | Some _, None -> None, chars
    | Some q, Some (r, g, b) -> Some (q*r, q*g, q*b), chars' in

  let parse_set chars =
    let get_color chars =
      let c, chars' = parse_color chars in
      let color = Option.value c ~default:(0, 0, 0) in
      color, chars' in

    let (r1, g1, b1), chars' = get_color chars in
    let _, chars' = parse_char ',' chars' in
    let (r2, g2, b2), chars' = get_color chars' in
    let _, chars' = parse_char ',' chars' in
    let (r3, g3, b3), chars' = get_color chars' in

    match (r1, g1, b1) with
    | (0, 0, 0) -> None, chars
    | _ -> Some (r1+r2+r3, g1+g2+g3, b1+b2+b3), chars' in

  let parse_sets chars =
    let rec helper chars acc = match (parse_char ';' *> parse_set) chars with
      | None, chars' -> acc, chars'
      | Some set, chars' -> helper chars' (set :: acc) in

    let first, chars' = parse_set chars in
    let sets = Option.to_list first in
    helper chars' sets in

  let chars = List.init (String.length line) (String.get line) in
  let game, chars = parse_start chars in
  let sets, _ = parse_sets chars in
  game, sets


let solve () =
  let maybe_read_line () = try Some (read_line ()) with End_of_file -> None in
  let rec solve_helper acc acc' = match maybe_read_line () with
    | None -> acc, acc'
    | Some line -> match parse_game line with
      | None, _ -> solve_helper acc acc'
      | Some game, sets ->
        let max_color (r, g, b) (r', g', b') = (max r r', max g g', max b b') in
        let (r, g, b) = List.fold_left max_color (0, 0, 0) sets in
        let ids = if r <= 12 && g <= 13 && b <= 14 then game + acc else acc in
        let power = r * g * b + acc' in
        solve_helper ids power in
  solve_helper 0 0

let () = let p1, p2 = solve () in
  Printf.printf "Part 1: %d\n" p1;
  Printf.printf "Part 2: %d\n" p2
