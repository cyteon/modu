[
  "let"
  "fn"
  "return"
  "if"
  "else"
  "for"
  "while"
  "loop"
  "class"
  "import"
  "as"
  "in"
  "and"
  "or"
] @keyword

(boolean) @constant.builtin
(null) @constant.builtin
(self) @variable.special
(string) @string
(number) @number
(comment) @comment

(break_stmt) @keyword
(continue_stmt) @keyword

(fn_stmt
  name: (identifier) @function)

(call_expr
  function: (identifier) @function)

; for <smth>.<smth>() calls
(call_expr
  function: (property_expr
    property: (identifier) @function.method))

(class_decl
  name: (identifier) @type)

(property_expr
  property: (identifier) @property)
 
(fn_stmt
  (identifier) @variable.parameter)

[
  "+"
  "-"
  "*"
  "/"
  "%"
  "**"
  "=="
  "!="
  "<"
  ">"
  "<="
  ">="
  "="
  "+="
  "-="
  "*="
  "/="
  "%="
  "!"
] @operator
 
["(" ")" "[" "]" "{" "}"] @punctuation.bracket
["," "." ":"] @punctuation.delimiter