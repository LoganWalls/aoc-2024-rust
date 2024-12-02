#!/usr/bin/env nu

def main [] {}

def "main fetch" [day: int] {
  http get https://adventofcode.com/2024/day/($day)/input --headers [cookie $env.AOC_KEY] | 
  str trim |
  save --progress --force $"($env.FILE_PWD)/inputs/day($day).txt"
}

def "main save-example-from-clipboard" [day: int] {
  pbpaste | str trim | save --progress --force $"($env.FILE_PWD)/inputs/day($day).example.txt"
}
