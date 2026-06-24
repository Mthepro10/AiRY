# AiRY
A programming language that uses an AI-assisted translation layer to convert natural language-like instructions into a structured intermediate representation, which is then compiled into executable code with rust interpreter. `Natural language -> AI translated code -> lexer -> parser -> Cranelift -> output`

## AiRY Core
This is the main part of the language (the actual programming). I will talk abot the syntax of the code later in the readME.

## code_wr.airy
This is the file where AiRY Core is written. This should be the exact name of the file.

## file.airy
This is were the AI part of the programming language goes in here the user writes his natural langauge that is then translated in AiRY Core using AI. This should be the exact name of the file.

## syntax of AiRY Core

`set` -- Declares a new dynamic variable, with an optional value. `set x 5`, `set x "Hello, World!"`, `set x 5.5`, or just `set x` to declare without assigning. Strings should always be in `""` not `''` .

`=` -- (a.k.a. `set_after`) Assigns a new value to an already-declared variable. `x = 5`, `x = perform y + 2`. Strings should always be in `""` not `''` .

`perform` -- Executes an arithmetic (`+ - * / %`), bitwise (`& | ^ ~`), or boolean (`&& || ! == != > < >= <=`) operation and returns a result that must be assigned or used. Arithmetic/bitwise/modulo operations always require `perform` (e.g. `set x perform 5 + 3`, `x = perform x & 2`, `show perform x % 2`). Boolean operations do **not** require `perform` and can be used directly in assignments or conditions (e.g. `set x 5 > 3`, `if x > 5 && y < 10 { ... }`). Parentheses are supported for grouping, e.g. `x = perform x + 1 + (1 + 1) * 8`.

`read` -- Reads a value from stdin into a variable. `read x`.

`show` -- Writes a variable's value (or the result of a `perform` expression) to stdout. `show x`, `show perform x + 2`.

`if`/`elseif`/`else` -- Conditional branching. `else` and `elseif` are optional but must always follow an `if`. Blocks are wrapped in `{ }` and may be written inline or with the contents on indented new lines:
```
if x > 5 {
    show x
} elseif x == 5 {
    show "x is 5"
} else {
    show 0
}
```

`loop` -- The equivalent of a `for` loop. Takes a variable name, a start value, and an end value (exclusive), executing the block for each value in that range. `loop i 0 10 { show i }` outputs 0 through 9. Loops can be nested inside loops, inside infloops, and combined with `if`/`elseif`/`else`.

`infloop` -- The equivalent of a `while` loop. Takes a condition and repeats the block while it holds. `infloop x == 5 { show "Hello, World!" , x = perform x + 1 }`. Can be nested with `loop` and `if` the same way `loop` can.

`break` -- Exits the current `loop` or `infloop` early. `loop i 0 10 { if i == 5 { break } , show i }`.

`return` -- Exits the main program early. Takes no parameters when called from the main function. `if x < 0 { show "Negative number" , return }`.

### Multiple statements per line
Any of the instructions above can be chained on a single line by separating them with commas, e.g. `set x perform 5 + 3, x = perform x * 2` or `loop i 0 10 { show i } , infloop x == 5 { show "Hello, World!" , x = perform x + 1 }`.

### Indentation
AiRY Core uses indentation to define blocks. Code inside `if`, `elseif`, `else`, `loop`, and `infloop` blocks should be indented four spaces from the enclosing instruction.

## some rules to know for file.airy

Since `file.airy` is parsed by the AI translation layer rather than the strict AiRY Core parser, the natural-language input follows a few conventions:

- **Clarifications**: wrap an aside to the AI translator in `? ?` to explain your intent without it ending up in the generated code. Example: `make x 5 ? this is to declare a variable x and assign it the value 5 ?` → `set x 5`.
- **Comments**: wrap comments in `! !`. These are preserved and appended after the instruction they annotate in the generated AiRY Core, e.g. `declare x and assign it 5 ! this helps me declare variable !` → `set x 5 ! this helps me declare variable !`. *(Currently disabled — see Warning below.)*
- **Indentation still matters**: even though you're writing natural language, the body of any `if`/`elseif`/`else`, `loop`, or `infloop` you describe must be indented (consistently, with either two or four spaces) relative to the structure it belongs to, so the translator can tell what's inside the block.
- **Synonyms & paraphrasing**: use whatever wording feels natural — "declare a variable" for `set`, "output" for `show`, "increment x by 1" for `x = perform x + 1`, etc. The translator maps intent to the closest AiRY Core instruction rather than matching exact keywords.
- **Pronouns**: words like "it" or "that variable" may refer back to a previously defined variable, and will be resolved automatically when unambiguous.
- **Multilingual input**: natural language input can be written in any language, as long as the intent is clear; the generated AiRY Core and any error/warning messages are always in English.
- **Typos**: small typos are tolerated and corrected automatically; typos large enough to change the meaning of an instruction are rejected. Variable/function names are exempt from typo-correction since they can be arbitrary.
- **Unsupported features / ambiguity**: if a request can't be mapped onto an AiRY Core instruction, is malformed, or the AI needs clarification, the translator stops and outputs only an error or clarifying question — no partial code and no extra commentary.
- **Warning**: `! !`-style comments are currently disabled in the translation layer and will be silently dropped (not treated as an error) rather than included in the output.

## What I want to do in the future
This programming language was built as a memory safe and easy syntax language. The AI translation part is  made so users do not have to always follow that strict syntax: hard languages like rust or c++. In the future I will love to extend my programming language (by creaating a ton of libraries), I want to create tons of AI prompts for diffrent librraries and even built a costum AI that can help the natural language translation to be made in miliseconds. Also do not forget you can also write in AiRY core if you want, just run the rust interpreter alone. 

## Sponsors
Still searching!!!

## UNSUPPORTED THINGS THAT I WANT TO INCLUDE IN THE MAIN LIBRARY
newline '\n', tab '\t', functions

## use of tab and newline
insted of `show "\n"` you need:
`show "
"`
or instead of `show "\t"` you need:
`show "    "`

## AI used
So for this project I used gemma-4-31B-it, but you can also use other types of agents, cus the prompt isn't made especially for gemma. But be aware that using huggingface consumes a number of your monthly credits from your account. That is why on the future I want to create a AI specially designed for this programming language and free for all.

## Example of AiRY and AiRY Core translation

```
Set sum to 0 and set count to 0.

Loop forever:
    Read a number into n.
    If n is 0, break out of the loop.
    If n is greater than 0, add n to sum and increase count by 1.

Show count.
Show sum.

If count is greater than 0:
    Set average to sum divided by count.
    Show average.
Else:
    Show "No positive numbers entered".

Create a loop starting from 0 up to sum and display each value. ?the loop shows every number?
```

```
set sum 0
set count 0

infloop true {
    read n
    if n == 0 {
        break
    }
    if n > 0 {
        sum = perform sum + n
        count = perform count + 1
    }
}

show count
show sum

if count > 0 {
    set average perform sum / count
    show average
} else {
    show "No positive numbers entered"
}

loop i 0 sum {
    show i
}
```
