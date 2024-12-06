@module("fs")
external readFileSync: (~name: string, [#utf8]) => string = "readFileSync"

let isDecr = x => {
  let res = ref(true)
  for i in 0 to x->Array.length - 2 {
    let delta = x[i + 1]->Option.getOr(0) - x[i]->Option.getOr(0)
    res := res.contents && (delta >= 1 && delta <= 3)
  }
  res.contents
}

let isIncr = x => {
  let res = ref(true)
  for i in 0 to x->Array.length - 2 {
    let delta = x[i]->Option.getOr(0) - x[i + 1]->Option.getOr(0)
    res := res.contents && (delta >= 1 && delta <= 3)
  }
  res.contents
}

let check = x => isDecr(x) || isIncr(x)

let solve = (case: string) => {
  let part1 =
    readFileSync(~name=`../data/day02/${case}.txt`, #utf8)
    ->String.split("\n")
    ->Array.map(x => x->String.trim)
    ->Array.filter(x => x->String.length > 0)
    ->Array.map(x =>
      x
      ->String.split(" ")
      ->Array.map(x => x->String.trim)
      ->Array.filter(x => x->String.length > 0)
      ->Array.map(x => x->Int.fromString->Option.getOr(0))
    )
    ->Array.map(x => x->check)
    ->Array.reduce(0, (x, acc) =>
      x + if acc {
        1
      } else {
        0
      }
    )
    ->Int.toString

  let part2 = 0->Int.toString

  Console.log(`Day 02, part 1, case '${case}':\t${part1}`)
  Console.log(`Day 02, part 2, case '${case}':\t${part2}`)
}

solve("test0")
solve("gh")
solve("google")
