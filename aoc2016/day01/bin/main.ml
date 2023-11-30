type move = L of int | R of int

let parse_move s =
  let steps = String.sub s 1 (String.length s - 1) |> int_of_string in
  match String.sub s 0 1 with
  | "R" -> R steps
  | "L" -> L steps
  | _ -> assert false;;

let parse_input input =
  String.split_on_char ',' input |> List.map String.trim |> List.map parse_move;;

let walk pos dir steps =
  let rec walk_helper (x, y) (dx, dy) steps acc =
    if steps = 0 then acc
    else let pos' = (x + dx, y + dy) in
      walk_helper pos' (dx, dy) (steps - 1) (pos' :: acc) in
  let positions = walk_helper pos dir steps [] in
  List.hd positions, List.rev positions;;

let left (dx, dy) = (dy, -dx);;
let right (dx, dy) = (-dy, dx);;
let dist (x, y) = abs x + abs y;;

let solve moves =
  let rec get_duplicate l =
    match l with
    | [] -> None
    | pos :: tl -> match List.find_opt (fun pos' -> pos = pos') tl with
      | None -> get_duplicate tl
      | d -> d in

  let rec solve_helper moves dir pos acc =
    match moves with
        | [] -> pos, get_duplicate acc
        | move :: moves' -> let dir', steps = match move with
            | L steps -> left dir, steps
            | R steps -> right dir, steps in
            let pos', positions = walk pos dir' steps in
            solve_helper moves' dir' pos' (List.append acc positions) in

  let p1, p2 = solve_helper moves (0, 1) (0, 0) [] in
  let d1, d2 = dist p1, Option.map dist p2 in
  let _ = Printf.printf "Part 1: %d\n" d1 in
  Option.iter (fun d2 -> Printf.printf "Part 2: %d\n" d2) d2;;

let () = read_line () |> parse_input |> solve;;
