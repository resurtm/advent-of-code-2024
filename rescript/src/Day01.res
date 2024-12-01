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
      | (m, n) => Math.abs(float(m - n))
      }
    )
    ->Array.reduce(0., (x, acc) => x +. acc)
    ->Float.toString

  Console.log(`Day 01, part 1, case '${case}':\t${part1}`)
}

solve("test0")
solve("gh")
solve("google")
