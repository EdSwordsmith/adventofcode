open Angstrom

(* Define let* operator for parser bind *)
let (let*) x f = x >>= f

let game =
  let integer = take_while1 (function '0' .. '9' -> true | _ -> false) >>| int_of_string in
  let start = string "Game " *> integer <* char ':' in
  let color =
    let red = string "red" *> return (1, 0, 0)
    and green = string "green" *> return (0, 1, 0)
    and blue = string "blue" *> return (0, 0, 1) in
    let name = red <|> green <|> blue in
    let* mult = char ' ' *> integer <* char ' ' in
    let* (r, g, b) = name in
    return (r * mult, g * mult, b * mult) in

  let set =
    let* first = color in
    let* colors = many (char ',' *> color) in
    let add_colors (r, g, b) (r', g', b') = r + r', g + g', b + b' in
    return @@ List.fold_left add_colors first colors in

  let sets =
    let* first = set in
    let* sets = many (char ';' *> set) in
    return @@ first :: sets in

  let* game = start in
  let* sets = sets in
  return (game, sets)

let solve () =
  let maybe_read_line () = try Some (read_line ()) with End_of_file -> None in
  let rec solve_helper acc acc' = match maybe_read_line () with
    | None -> acc, acc'
    | Some line -> match parse_string ~consume:All game line with
      | Error _ -> solve_helper acc acc'
      | Ok (game, sets) ->
        let max_color (r, g, b) (r', g', b') = max r r', max g g', max b b' in
        let r, g, b = List.fold_left max_color (0, 0, 0) sets in
        let ids = if r <= 12 && g <= 13 && b <= 14 then game + acc else acc in
        let power = r * g * b + acc' in
        solve_helper ids power in
  solve_helper 0 0

let () = let p1, p2 = solve () in
  Printf.printf "Part 1: %d\n" p1;
  Printf.printf "Part 2: %d\n" p2
