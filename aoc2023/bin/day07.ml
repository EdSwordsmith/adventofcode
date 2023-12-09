type hand = Five | Four | Full | Three | TwoPair | Pair | High

module CMap = Map.Make (Char)

let compare_cards c1 c2 =
  let card_to_int card =
    match card with
    | '2' .. '9' -> int_of_char card - 48
    | 'T' -> 10
    | 'J' -> 11
    | 'Q' -> 12
    | 'K' -> 13
    | 'A' -> 14
    | _ -> assert false
  in
  compare (card_to_int c1) (card_to_int c2)

let compare_cards' c1 c2 =
  let card_to_int card =
    match card with
    | 'J' -> 1
    | '2' .. '9' -> int_of_char card - 48
    | 'T' -> 10
    | 'Q' -> 12
    | 'K' -> 13
    | 'A' -> 14
    | _ -> assert false
  in
  compare (card_to_int c1) (card_to_int c2)

let cmp_hands cmp_card (h1, h1') (h2, h2') =
  let hand_to_int hand =
    match hand with
    | Five -> 6
    | Four -> 5
    | Full -> 4
    | Three -> 3
    | TwoPair -> 2
    | Pair -> 1
    | High -> 0
  in

  let rec cmp_cards h1 h2 =
    match (h1, h2) with
    | [], [] -> 0
    | c :: h1, c' :: h2 -> (
        match cmp_card c c' with 0 -> cmp_cards h1 h2 | n -> n)
    | _, _ -> assert false
  in

  match compare (hand_to_int h1) (hand_to_int h2) with
  | 0 -> cmp_cards h1' h2'
  | n -> n

let str_to_hand s =
  let chars = List.init (String.length s) (String.get s) in
  let rec aux chars map =
    match chars with
    | [] -> map
    | c :: tl ->
        let count = CMap.find_opt c map |> Option.value ~default:0 in
        let map = CMap.add c (count + 1) map in
        aux tl map
  in

  let map_to_hand map =
    let cards =
      CMap.to_seq map |> List.of_seq
      |> List.sort (fun (c, q) (c', q') ->
             match compare q' q with 0 -> compare_cards c' c | comp -> comp)
    in

    match cards with
    | [ (_, 5) ] -> Five
    | [ (_, 4); _ ] -> Four
    | [ (_, 3); (_, 2) ] -> Full
    | [ (_, 3); _; _ ] -> Three
    | [ (_, 2); (_, 2); _ ] -> TwoPair
    | [ (_, 2); _; _; _ ] -> Pair
    | [ (_, 1); _; _; _; _ ] -> High
    | _ -> assert false
  in

  let map_to_hand' map =
    let map' = CMap.filter (fun c _ -> c <> 'J') map in
    let cards =
      CMap.to_seq map' |> List.of_seq
      |> List.sort (fun (c, q) (c', q') ->
             match compare q' q with 0 -> compare_cards c' c | comp -> comp)
    in
    let js = CMap.find_opt 'J' map |> Option.value ~default:0 in

    (match cards with [] -> map | (c, q) :: _ -> CMap.add c (q + js) map')
    |> map_to_hand
  in

  let card_map = aux chars CMap.empty in
  let hand = map_to_hand card_map in
  let hand' = map_to_hand' card_map in

  ((hand, chars), (hand', chars))

let split_string str = String.split_on_char ' ' str

let parse_line line =
  match split_string line with
  | [ hand; bid ] ->
      let h, h' = str_to_hand hand in
      let bid = int_of_string bid in
      ((h, bid), (h', bid))
  | _ -> assert false

let solve () =
  let next_line () = try Ok (read_line ()) with End_of_file -> Error "" in
  let rec lines () =
    match next_line () with
    | Ok line -> Seq.Cons (line, lines)
    | Error _ -> Seq.Nil
  in

  let input = Seq.map parse_line lines |> List.of_seq in
  let input' = List.map snd input in
  let input = List.map fst input in

  let aux cmp input =
    input
    |> List.sort (fun (h1, _) (h2, _) -> cmp_hands cmp h1 h2)
    |> List.mapi (fun i (_, bid) -> (i + 1) * bid)
    |> List.fold_left ( + ) 0
  in

  (aux compare_cards input, aux compare_cards' input')

let () =
  let p1, p2 = solve () in
  Printf.printf "Part 1: %d\n" p1;
  Printf.printf "Part 2: %d\n" p2
