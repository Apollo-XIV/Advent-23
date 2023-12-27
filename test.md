# day 5 part 2
## Problem Spec
Given a set of ranges, take each one and pass them through a conversion map.

Each conversion map consists of a range of numbers that it applies to, and then the difference by which to modify the ranges.

for each: seed range (after each conversion block, extra seeds are added to the list)
  for each: conversion block -> Vec<seed ranges>
    extract valid seed ranges for each row of the conversion block
    keep going until the unprocessed queue is empty


types of range overlap

< > = source range
[ ] = comparisson range

<[]> // total overlap
<[>] // partial overlap
[<]> // partial overlap
[<>] // total overlap
<>[] // no overlap