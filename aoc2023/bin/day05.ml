open Angstrom

module Range = struct
  let from_seeds = List.map (fun s -> (s, 1))

  let from_seeds' seeds =
    let rec aux seeds acc =
      match seeds with s :: l :: tl -> aux tl ((s, l) :: acc) | _ -> acc
    in
    aux seeds []

  let inter (s, l) (s', l') =
    let s2 = max s s' in
    let e = -1 + min (s + l) (s' + l') in
    if s2 > e then None else Some (s2, e - s2 + 1)

  let subtract (s1, l1) (s2, l2) =
    let end1 = s1 + l1 - 1 in
    let end2 = s2 + l2 - 1 in

    if end1 < s2 || s1 > end2 then [ (s1, l1) ]
    else if s1 < s2 && end1 > end2 then
      [ (s1, s2 - s1); (end2 + 1, end1 - end2) ]
    else if s1 < s2 then [ (s1, s2 - s1) ]
    else [ (end2 + 1, end1 - end2) ]

  let apply (dst, src, len) range =
    match inter range (src, len) with
    | None -> (None, [ range ])
    | Some (s, l) ->
        let remaining =
          subtract range (s, l) |> List.filter (fun (_, l) -> l > 0)
        in
        (Some (s - src + dst, l), remaining)
end

let parse_input () =
  let ( let* ) x f = x >>= f in
  let ( let+ ) = Result.bind in

  let integer =
    take_while1 (function '0' .. '9' -> true | _ -> false) >>| int_of_string
  in

  let parse_seeds line =
    let parser = string "seeds:" *> many (char ' ' *> integer) in
    parse_string ~consume:All parser line
  in

  let parse_mapping line =
    let parser =
      let* dst = integer in
      let* src = char ' ' *> integer in
      let* len = char ' ' *> integer in
      return (dst, src, len)
    in
    parse_string ~consume:All parser line
  in

  let next_line () = try Ok (read_line ()) with End_of_file -> Error "" in

  let+ start = next_line () in
  let+ seeds = parse_seeds start in
  let+ _ = next_line () in

  let parse_map () =
    let _ = next_line () in
    let rec aux mappings =
      let+ line = next_line () in
      match line with
      | "" -> Ok mappings
      | _ ->
          let+ mapping = parse_mapping line in
          aux (mapping :: mappings)
    in
    aux []
  in

  let parse_maps () =
    let rec aux () maps =
      match parse_map () with
      | Error _ -> maps
      | Ok map -> map :: maps |> aux ()
    in
    aux () []
  in

  let maps = parse_maps () |> List.rev in

  Ok (seeds, maps)

let solve in_seeds maps =
  let seeds = in_seeds |> Range.from_seeds in
  let seeds' = in_seeds |> Range.from_seeds' in

  let apply_map mappings ranges =
    let rec aux mappings ranges ranges' =
      match mappings with
      | [] -> ranges @ ranges'
      | mapping :: tl ->
          let res = List.map (Range.apply mapping) ranges in
          let ranges = List.map (fun (_, rest) -> rest) res |> List.flatten in
          let new_ranges =
            List.map (fun (n, _) -> Option.to_list n) res |> List.flatten
          in
          aux tl ranges (new_ranges @ ranges')
    in
    aux mappings ranges []
  in
  let apply_maps maps ranges =
    let rec aux maps ranges =
      match maps with
      | [] -> ranges
      | map :: tl ->
          let ranges = apply_map map ranges in
          aux tl ranges
    in
    aux maps ranges
  in

  let min_ranges ranges =
    let nums = List.map (fun (n, _) -> n) ranges in
    let n = List.nth nums 0 in
    List.fold_left min n nums
  in
  let final = apply_maps maps seeds in
  let p1 = min_ranges final in

  let final' = apply_maps maps seeds' in
  let p2 = min_ranges final' in
  (p1, p2)

let () =
  match parse_input () with
  | Error _ -> print_endline "Sorry :("
  | Ok (seeds, maps) ->
      let p1, p2 = solve seeds maps in
      Printf.printf "Part 1: %d\n" p1;
      Printf.printf "Part 2: %d\n" p2
