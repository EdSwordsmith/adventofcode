defmodule Interpreter do
  defstruct paused: false, v1: 0, v2: 0

  def parse_expr(expr) do
    case expr do
      ["do()"] -> :do
      ["don't()"] -> :dont
      [_, fst, snd] -> String.to_integer(fst) * String.to_integer(snd)
    end
  end

  def eval(exprs, interpreter \\ %Interpreter{})

  def eval([:do | exprs], interpreter) do
    eval(exprs, %{interpreter | paused: false})
  end

  def eval([:dont | exprs], interpreter) do
    eval(exprs, %{interpreter | paused: true})
  end

  def eval([value | exprs], %{paused: true, v1: v1} = interpreter) do
    eval(exprs, %{interpreter | v1: v1 + value})
  end

  def eval([value | exprs], %{paused: false, v1: v1, v2: v2} = interpreter) do
    eval(exprs, %{interpreter | v1: v1 + value, v2: v2 + value})
  end

  def eval([], interpreter) do
    interpreter
  end
end

exprs =
  ~r/do\(\)|don't\(\)|mul\((?<fst>\d+),(?<snd>\d+)\)/
  |> Regex.scan(IO.read(:eof), captures: ["fst", "snd"])
  |> Enum.map(&Interpreter.parse_expr/1)

result = Interpreter.eval(exprs)

IO.puts("Part 1: #{result.v1}")
IO.puts("Part 2: #{result.v2}")
