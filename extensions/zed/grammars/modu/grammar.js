module.exports = grammar({
    name: "modu",
    rules: {
        program: ($) => repeat($._expression),

        _expression: ($) =>
            choice(
                $.identifier,
                $.number,
                $.string,
            ),

        identifier: ($) => /[a-zA-Z_][a-zA-Z0-9_]*/,

        number: ($) => /\d+(\.\d+)?/,

        string: ($) =>
            choice(
                seq('"', /[^"]*/, '"'),
                seq("'", /[^']*/, "'"),
            ),
    }
});