open Angstrom
module Graph = Map.Make (String)

type dir = Left | Right

let parse_input () =
  let ( let+ ) = Result.bind in

  let dir =
    let right = char 'R' >>| fun _ -> Right in
    let left = char 'L' >>| fun _ -> Left in
    right <|> left
  in

  let node =
    let ( let* ) x f = x >>= f in
    let edges =
      let* left = char '(' *> take 3 in
      let* right = string ", " *> take 3 <* char ')' in
      return (left, right)
    in

    let* name = take 3 <* string " = " in
    let* edges' = edges in
    return (name, edges')
  in

  let line () = try Some (read_line ()) with End_of_file -> None in
  let rec aux map =
    match line () with
    | None -> Ok map
    | Some line ->
        let+ name, edges = parse_string ~consume:All node line in
        let map = Graph.add name edges map in
        aux map
  in

  let+ dirs = read_line () |> parse_string ~consume:All (many dir) in
  let _ = read_line () in
  let+ graph = aux Graph.empty in

  Ok (dirs, graph)

let next graph dir node =
  let l, r = Graph.find node graph in
  match dir with Left -> l | Right -> r

let rec gcd a b = if b = 0 then a else gcd b (a mod b)
let lcm a b = if a = 0 || b = 0 then 0 else abs (a * b) / gcd a b

let rec lcm_of_list = function
  | [] -> 1
  | [ x ] -> x
  | x :: xs -> lcm x (lcm_of_list xs)

let solve (dirs, graph) =
  let dir_len = List.length dirs in
  let get_dir dirs n =
    let index = Int.rem n dir_len in
    List.nth dirs index
  in

  let rec aux dirs graph pos steps =
    match pos with
    | "ZZZ" -> steps
    | _ ->
        let dir = get_dir dirs steps in
        let pos = next graph dir pos in
        aux dirs graph pos (steps + 1)
  in

  let rec aux' dirs graph steps pos =
    if String.ends_with ~suffix:"Z" pos then steps
    else
      let dir = get_dir dirs steps in
      let pos = next graph dir pos in
      aux' dirs graph (steps + 1) pos
  in

  let p1 =
    if Graph.mem "AAA" graph then Some (aux dirs graph "AAA" 0) else None
  in

  let start =
    Graph.to_seq graph |> Seq.map fst
    |> Seq.filter (String.ends_with ~suffix:"A")
    |> List.of_seq
  in
  let p2 = start |> List.map (aux' dirs graph 0) |> lcm_of_list in

  (p1, p2)

let () =
  let p1, p2 = parse_input () |> Result.get_ok |> solve in
  Option.iter (Printf.printf "Part 1: %d\n") p1;
  Printf.printf "Part 2: %d\n" p2
