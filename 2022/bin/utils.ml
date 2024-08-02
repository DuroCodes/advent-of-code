type answer = Answer of int | Unknown

let answer_to_string = function
  | Answer x -> Int.to_string x
  | Unknown -> "Not Yet Implemented"
