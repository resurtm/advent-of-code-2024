@module("fs")
external readFileSync: (~name: string, [#utf8]) => string = "readFileSync"

let solve = (case: string) => {
  let a = []
  let b = []

  readFileSync(~name=`../data/day01/${case}.txt`, #utf8)
  ->String.split("\n")
  ->Array.map(x => x->String.trim)
  ->Array.filter(x => x->String.length > 0)
  ->Array.map(x =>
    x
    ->String.split(" ")
    ->Array.map(y => y->String.trim)
    ->Array.filter(y => y->String.length > 0)
  )
  ->Array.forEach(x =>
    switch x {
    | [m, n] => {
        a->Array.push(m->Int.fromString->Option.getExn)
        b->Array.push(n->Int.fromString->Option.getExn)
      }
    | _ => ()
    }
  )

  a->Array.sort((m, n) => float(m - n))
  b->Array.sort((m, n) => float(m - n))

  let part1 =
    Belt.Array.zip(a, b)
    ->Array.map(x =>
      switch x {
      | (m, n) => Math.abs(float(m - n))->Int.fromFloat
      }
    )
    ->Array.reduce(0, (acc, x) => acc + x)
    ->Int.toString

  let freq =
    b->Array.reduce(Belt.Map.Int.empty, (acc, x) =>
      acc->Belt.Map.Int.set(x, acc->Belt.Map.Int.getWithDefault(x, 0) + 1)
    )
  let part2 =
    a
    ->Array.map(x => freq->Belt.Map.Int.getWithDefault(x, 0) * x)
    ->Array.reduce(0, (acc, x) => acc + x)
    ->Int.toString

  Console.log(`Day 01, part 1, case '${case}':\t${part1}`)
  Console.log(`Day 01, part 2, case '${case}':\t${part2}`)
}

solve("test0")
solve("gh")
solve("google")
