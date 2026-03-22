const NEWLINE = /\r?\n/;


module.exports = grammar({
    name: "modu",
    rules: {
        identifier: ($) => /[a-zA-Z_][a-zA-Z0-9_]*/,

        number: ($) => /\d+(\.\d+)?/,

        string: ($) =>
            choice(
                seq('"', /[^"]*/, '"'),
                seq("'", /[^']*/, "'"),
            ),
    }
});