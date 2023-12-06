open Angstrom

let parse_input () =
  let ( let* ) = Result.bind in
  let integer =
    take_while1 (function '0' .. '9' -> true | _ -> false) >>| int_of_string
  and space = many (char ' ') in
  let numbers prefix = string prefix *> many (space *> integer) in

  let* times = read_line () |> parse_string ~consume:All (numbers "Time:") in
  let* distances =
    read_line () |> parse_string ~consume:All (numbers "Distance:")
  in

  let list_to_int list =
    list |> List.map string_of_int |> List.fold_left ( ^ ) "" |> int_of_string
  in

  let time = list_to_int times in
  let distance = list_to_int distances in

  Ok (List.combine times distances, (time, distance))

let solve (races, race) =
  let win (t, d) =
    let t', d' = (Int.to_float t, Int.to_float d) in
    let t2 = t' *. t' in
    let inner = t2 -. (4.0 *. d') in
    let first = (t' -. sqrt inner) /. 2.0 |> Float.floor |> Int.of_float in
    let last = (t' +. sqrt inner) /. 2.0 |> Float.ceil |> Int.of_float in
    last - first - 1
  in

  let p1 = races |> List.map win |> List.fold_left ( * ) 1 in
  let p2 = win race in

  (p1, p2)

let () =
  let p1, p2 = parse_input () |> Result.get_ok |> solve in
  Printf.printf "Part 1: %d\n" p1;
  Printf.printf "Part 2: %d\n" p2
