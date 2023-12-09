open Angstrom

let parse_line line =
  let integer =
    take_while1 (function '-' -> true | '0' .. '9' -> true | _ -> false)
    >>| int_of_string
  in
  let spaces = many (char ' ') in
  let numbers = many (spaces *> integer) in
  parse_string ~consume:All numbers line

let rec derivate l =
  match l with
  | [] -> []
  | h :: tl -> ( match tl with [] -> [] | h' :: _ -> (h' - h) :: derivate tl)

let rec predict l =
  if List.for_all (( = ) 0) l then 0
  else
    let l' = derivate l |> predict in
    let last = List.length l - 1 |> List.nth l in
    last + l'

let rec predict' l =
  if List.for_all (( = ) 0) l then 0
  else
    let l' = derivate l |> predict' in
    let first = List.nth l 0 in
    first - l'

let solve () =
  let next_line () = try Ok (read_line ()) with End_of_file -> Error "" in
  let rec lines () =
    match next_line () with
    | Ok line -> Seq.Cons (line, lines)
    | Error _ -> Seq.Nil
  in

  lines |> Seq.map parse_line |> Seq.map Result.get_ok
  |> Seq.map (fun x -> (predict x, predict' x))
  |> Seq.fold_left (fun (x, x') (y, y') -> (x + y, x' + y')) (0, 0)

let () =
  let p1, p2 = solve () in
  Printf.printf "Part 1: %d\n" p1;
  Printf.printf "Part 2: %d\n" p2
