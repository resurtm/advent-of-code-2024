package com.resurtm.aoc24.day01

import java.io.File
import kotlin.math.abs

fun solveDay01() {
    solveInternal("test0.txt")
    solveInternal("gh.txt")
    solveInternal("google.txt")
}

private fun solveInternal(testCase: String) {
    val a = mutableListOf<Int>()
    val b = mutableListOf<Int>()
    val c = mutableMapOf<Int, Int>()

    File("../../data/day01/$testCase")
        .readLines()
        .map { it.trim() }
        .filter { it.isNotEmpty() }
        .forEach { line ->
            val (l, r) = line.split(" ")
                .map { it.trim() }
                .filter { it.isNotEmpty() }
                .map { it.toInt() }
            a.add(l)
            b.add(r)
        }

    a.sort()
    b.sort()

    var part1 = 0
    var part2 = 0

    repeat(a.size) {
        part1 += abs(a[it] - b[it])
        c[b[it]] = c.getOrDefault(b[it], 0) + 1
    }
    repeat(a.size) {
        part2 += a[it] * c.getOrDefault(a[it], 0)
    }

    println("-".repeat(30))
    println("Test Case:\t$testCase")
    println("Day 01, Part 1:\t$part1")
    println("Day 01, Part 2:\t$part2")
}
