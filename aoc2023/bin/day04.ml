open Angstrom

(* let binding operator por the parser binding *)
let ( let* ) x f = x >>= f

module ISet = Set.Make (Int)
module IMap = Map.Make (Int)

let pow base exponent =
  let rec helper base exponent acc =
    if exponent = 0 then acc else helper base (exponent - 1) (base * acc)
  in
  helper base exponent 1

let rec range i j = if j < i then [] else i :: range (i + 1) j

let parse_line line =
  let integer =
    take_while1 (function '0' .. '9' -> true | _ -> false) >>| int_of_string
  and spaces = many (char ' ') in

  let number = spaces *> integer <* spaces in

  let start = string "Card" *> number <* char ':'
  and numbers =
    let* nums = many number in
    return @@ ISet.of_list nums
  in

  let card =
    let* gn = start in
    let* winning = numbers in
    let* nums = char '|' *> numbers in
    return (gn, winning, nums)
  in

  parse_string ~consume:All card line

let parse_input () =
  let maybe_read_line () = try Some (read_line ()) with End_of_file -> None in
  let rec helper cards =
    match maybe_read_line () with
    | None -> cards
    | Some line -> (
        match parse_line line with
        | Ok card -> helper (card :: cards)
        | _ -> helper cards)
  in
  helper [] |> List.rev

let solve () =
  let card_points matches = if matches = 0 then 0 else pow 2 (matches - 1) in

  let add_cards map card_n n =
    IMap.mapi
      (fun card_n' amt ->
        if card_n' <= card_n + n && card_n' > card_n then
          amt + IMap.find card_n map
        else amt)
      map
  in

  let rec helper input points cards =
    match input with
    | [] ->
        let total_cards = IMap.fold (fun _ -> ( + )) cards 0 in
        (points, total_cards)
    | (cn, winning, numbers) :: input' ->
        let matches = ISet.inter winning numbers |> ISet.cardinal in
        let points' = card_points matches in
        let cards' = add_cards cards cn matches in
        helper input' (points + points') cards'
  in

  let input = parse_input () in
  let cards =
    range 1 (List.length input)
    |> List.map (fun n -> (n, 1))
    |> List.to_seq |> IMap.of_seq
  in
  helper input 0 cards

let () =
  let p1, p2 = solve () in
  Printf.printf "Part 1: %d\n" p1;
  Printf.printf "Part 2: %d\n" p2
