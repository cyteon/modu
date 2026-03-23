module.exports = grammar({
    name: "modu",

    extras: ($) => [/\s/, $.comment],
    word: ($) => $.identifier,

    rules: {
        program: ($) => repeat($._statement),

        _statement: ($) =>
            choice(
                $.let_stmt,
                $.const_stmt,
                $.assign_stmt,
                $.fn_stmt,
                $.if_stmt,
                $.for_stmt,
                $.while_stmt,
                $.loop_stmt,
                $.class_decl,
                $.return_stmt,
                $.break_stmt,
                $.continue_stmt,
                $.import_stmt,
                $._expression,
            ),
        
        let_stmt: ($) => 
            seq("let", $.identifier, "=", $._expression),
        const_stmt: ($) =>
            seq("const", $.identifier, "=", $._expression),
        assign_stmt: ($) =>
            seq($.identifier, choice("=", "+=", "-=", "*=", "/=", "%="), $._expression),
        fn_stmt: ($) => 
            seq("fn", field("name", $.identifier), "(", optional(commaSeperated($.identifier)), ")", $._block),
        if_stmt: ($) =>
            seq(
                "if", $._expression, $._block, 
                repeat(seq(seq("else", "if"), $._expression, $._block)), 
                optional(seq("else", $._block))
            ),
        for_stmt: ($) =>
            seq("for", field("name", $.identifier), "in", $._expression, $._block),
        while_stmt: ($) =>
            seq("while", $._expression, $._block),
        loop_stmt: ($) =>
            seq("loop", $._block),
        class_decl: ($) =>
            seq("class", field("name", $.identifier), "{", repeat($.fn_stmt), "}"),
        return_stmt: ($) =>
            prec.right(seq("return", optional($._expression))),
        break_stmt: ($) => 
            "break",
        continue_stmt: ($) => 
            "continue",
        import_stmt: ($) =>
            seq("import", $.string, optional(seq("as", $.identifier))),

        _block: ($) => seq("{", repeat($._statement), "}"),

        _expression: ($) =>
            choice(
                $.identifier,
                $.number,
                $.string,
                $.boolean,
                $.null,
                $.self,
                $.unary_expr,
                $.binary_expr,
                $.call_expr,
                $.property_expr,
                $.index_expr,
                $.array_literal,
                $.object_literal,
                $.parens_expr,
            ),

        identifier: ($) => /[a-zA-Z_][a-zA-Z0-9_]*/,

        number: ($) => /\d+(\.\d+)?/,

        string: ($) =>
            choice(
                seq('"', /[^"]*/, '"'),
                seq("'", /[^']*/, "'"),
            ),
        
        boolean: ($) =>
            choice("true", "false"),
    
        null: ($) => "null",
        self: ($) => "self",

        comment: ($) =>
            choice(
                seq("//", /[^\n]*/),
                seq("/*", /[^*]*\*+([^/*][^*]*\*+)*/, "/"),
            ),
        
        unary_expr: ($) =>
            seq(choice("!", "-", "~"), $._expression),
    
        binary_expr: ($) =>
            choice(
                prec.left(1, seq($._expression, choice("and", "or"), $._expression)),
                prec.left(2, seq($._expression, choice("==", "!=", "<", ">", "<=", ">="), $._expression)),
                prec.left(3, seq($._expression, choice("&", "|", "^", "<<", ">>"), $._expression)),
                prec.left(4, seq($._expression, choice("+", "-"), $._expression)),
                prec.left(5, seq($._expression, choice("*", "/", "%"), $._expression)),
                prec.left(6, seq($._expression, "**", $._expression))
            ),
        
        call_expr: ($) =>
            prec.left(10, seq(field("function", $._expression), "(", optional(commaSeperated($._expression)), ")")),
    
        property_expr: ($) =>
            prec.left(10, seq($._expression, ".", field("property", $.identifier))),
    
        index_expr: ($) =>
            prec.left(10, seq($._expression, "[", $._expression, "]")),
    
        array_literal: ($) =>
            seq("[", optional(commaSeperated($._expression)), "]"),
    
        object_literal: ($) =>
            seq("{", optional(commaSeperated(seq($.identifier, ":", $._expression))), "}"),

        parens_expr: ($) =>
            seq("(", $._expression, ")"),
    }
});

function commaSeperated(rule) {
  return seq(rule, repeat(seq(",", rule)), optional(","));
}