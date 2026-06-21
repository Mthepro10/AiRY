PROMPT_TEMPLATE = """
Convert this into STRICT AiRY.
AiRY is an AI-assisted programming language designed to be simple, intuitive, and accessible to everyone. Users write code in natural language, and your task is to translate it into valid AiRY Core code. 
To be more precise, you will receive a piece of code written in a natural language format. Your job is to convert this code into AiRY Core, which is the underlying language that AiRY uses to execute commands.

Till now the llanguage has been evolving and we have been adding new features to it. So, the code you receive might contain some features that are not yet supported in AiRY Core. In such cases, your answer should be just an error message indicating that the feature is not supported, without any additional explanations or text.

I will now tell you all posible instructions from AiRY, and you will have to use only these instructions to convert the given code into AiRY Core. You should not use any other instructions or features that are not listed below. If you encounter any feature in the input code that is not listed below, you should return an error message indicating that the feature is not supported.

[
read - This instruction is used to read from stdin; It takes a variable name as an argument and assigns the input value to that variable.
show - This instruction is used to write to stdout; It takes a variable name as an argument and outputs the value of that variable.
set - This instruction is used to declare a new dynamic variable and assign a value to it; It takes a variable name and a value as arguments and assigns the value to the variable. Used like this set x 5 , or also set x "Hello, World!" , or also set x 5.5 . Also it can be used  without the other value, just set x to declare a variable without assigning a value to it.
operations - AiRY supports basic arithmetic operations like addition (+), subtraction (-), multiplication (*), and division (/). You can use these operations to perform calculations on variables and values. For example, you can write set x perform 5 + 3 to assign the result of the addition to variable x.
perform an operation - This instruction is used to perform an operation on variables and return the result. The result must be assigned to a variable or used in an expression. For example, you can write x = perform x + 2 to add 2 to the current value of x and assign the result back to x. Or you can write show perform x + 2 to output the result of the addition.
bitwise operations - AiRY also supports bitwise operations like AND (&), OR (|), XOR (^) , and NOT (~). You can use these operations to perform bitwise calculations on variables and values. For example, you can write set x perform 5 & 3 to assign the result of the bitwise AND operation to variable x.
performing a bitwise operation - This instruction is used to perform a bitwise operation on variables and return the result. The result must be assigned to a variable or used in an expression. For example, you can write x = perform x & 2 to perform a bitwise AND operation between the current value of x and 2, and assign the result back to x. Or you can write show perform x & 2 to output the result of the operation.
modulo operation - AiRY also supports the modulo operation (%), which gives the remainder of a division operation. You can use this operation to perform modulo calculations on variables and values. For example, you can write set x perform 5 % 3 to assign the result of the modulo operation to variable x.
performing a modulo operation - This instruction is used to perform a modulo operation on variables and return the result. The result must be assigned to a variable or used in an expression. For example, you can write x = perform x % 2 to perform a modulo operation between the current value of x and 2, and assign the result back to x. Or you can write show perform x % 2 to output the result of the operation.
more operations on one line - You can perform multiple operations on the same line by separating them with commas. For example, you can write set x perform 5 + 3, x = perform x * 2 to first assign the result of the addition to variable x, and then multiply x by 2 and assign the result back to x. This method can be used for any kind of operations or functions, like below you will have below functions like infloop, loop, if, else, and elseif, you can write multiple of them on the same line by separating them with commas. For example, you can write if x > 5 {{ show x }} , elseif x == 5 {{ show "x is 5" }} , else {{ show 0 }} to create a conditional statement with multiple branches on the same line; same thing can be done with loops and infloops. Like you can write loop i 0 10 {{ show i }} , loop j 0 5 {{ show j }} to create two loops on the same line; or you can combine them like this loop i 0 10 {{ show i }} , infloop x == 5 {{ show "Hello, World!" , x = perform x + 1 }} to create a loop and an infloop on the same line.
if - This instruction is used for conditional statements; It takes a condition and a block of code as arguments and executes the block of code if the condition is true. For example, you can write if x > 5 {{ show x }} to output the value of x if it is greater than 5. It can be written also the instructions inside the brackets in a new line, like this:

if x > 5 {{
    show x
}}

else - This instruction is used for conditional statements; It takes a condition and two blocks of code as arguments and executes the first block of code if the condition is true, and the second block of code if the condition is false. For example, you can write if x > 5 {{ show x }} else {{ show 0 }} to output the value of x if it is greater than 5, and output 0 otherwise. It can be written also the instructions inside the brackets in a new line, like this:

if x > 5 {{
    show x
}}
else {{
    show 0
}}

It should always follow an if statement, but else is optional.

elseif - This instruction is used for conditional statements; It takes a condition and a block of code as arguments and executes the block of code if the condition is true. It can be used in combination with if and else to create multiple branches of conditional statements. For example, you can write:

if x > 5 {{
    show x
}} elseif x == 5 {{
    show "x is 5"
}} else {{
    show 0
}}

boolean operations - AiRY supports boolean operations like AND (&&), OR (||), NOT (!), EQUAL (==), NOT_EQUAL (!=), GREATER_THAN (>), LESS_THAN (<), GREATER_THAN_EQUAL (>=), LESS_THAN_EQUAL (<=).
use boolean operations - They can be used in conditional statements to create complex conditions. For example, you can write if x > 5 && y < 10 {{ show x }} to output the value of x if it is greater than 5 and the value of y is less than 10. Another example is if !(x == 5) {{ show "x is not 5" }} to output "x is not 5" if x is not equal to 5. Or you can also use it as a set operation like set x 5 > 3 to assign the result of the boolean operation to variable x.
set_after or = - Unlike set x y wich is to declare variables and (optional) assign values to them, set_after or = is used to assign a new value to an already declared variable. It takes a variable name, an operator, and another variable or value as arguments and updates the variable with the new value. For example, you can write x = 5 to assign the value 5 to variable x, or you can write x = perform y + 2 to assign the result of the addition of y and 2 to variable x. In actual code just = will be used instead of set_after, but I am mentioning set_after here to make it clear that this is a different instruction from set, and it is used for a different purpose.
loop - This instruction is the equivalent of a for loop; It takes a variable name, a start value, an end value, and a block of code as arguments and executes the block of code for each value of the variable from the start value to the end value. For example, you can write loop i 0 10 {{ show i }} to output the values from 0 to 9. It can be written also the instructions inside the brackets in a new line, like this:
loop i 0 10 {{
    show i
}}

infloop - This instruction is the equivalent of a while loop; It takes a condition and a block of code as arguments and executes the block of code repeatedly as long as the condition is true. For example, you can write set x 5, infloop x == 5 {{ show "Hello, World!" , x = perform x + 1 }} to output "Hello, World!" indefinitely. It can be written also the instructions inside the brackets in a new line, like this:
set x 5
infloop x==5 {{
    show "Hello, World!"
    x = perform x + 1
}}

loop in loop - You can also have nested loops by placing one loop inside another. For example, you can write loop i 0 10 {{ show i , loop j 0 5 {{ show j }} }} to create a nested loop that outputs the values of i and j. This can be done multiple times to create multiple levels of nested loops. For example, you can write loop i 0 10 {{ show i , loop j 0 5 {{ show j , loop k 0 3 {{ show k }} }} }} to create a nested loop with three levels that outputs the values of i, j, and k.
loop in infloop or infloop in loop - You can also have a loop inside an infloop or an infloop inside a loop. For example, you can write infloop x == 5 {{ show "Hello, World!" , loop i 0 10 {{ show i }} }} to create an infinite loop that outputs "Hello, World!" and then outputs the values from 0 to 9 indefinitely. This can be done multiple times to create multiple levels of nested loops and infloops. For example, you can write infloop x == 5 {{ show "Hello, World!" , loop i 0 10 {{ show i , infloop y < 3 {{ show y , y = perform y + 1 }} }} }} to create an infinite loop that outputs "Hello, World!" and then outputs the values from 0 to 9, and for each value of i it also outputs the values from 0 to 2 indefinitely.
loop with if - You can also have conditional statements inside loops. For example, you can write loop i 0 10 {{ if i % 2 == 0 {{ show i }} }} to create a loop that outputs only the even values of i. This can be done multiple times to create more complex loops with multiple conditional statements. For example, you can write loop i 0 10 {{ if i % 2 == 0 {{ show i }} elseif i % 3 == 0 {{ show "Divisible by 3" }} else {{ show "Other" }} }} to create a loop that outputs the even values of i, outputs "Divisible by 3" for values of i that are divisible by 3, and outputs "Other" for all other values of i.
infloop with if - You can also have conditional statements inside infloops. For example, you can write infloop x == 5 {{ if x > 5 {{ show "Greater than 5" }} elseif x == 5 {{ show "Equal to 5" }} else {{ show "Less than 5" }} , x = perform x + 1 }} to create an infinite loop that outputs whether x is greater than, equal to, or less than 5, and then increments x indefinitely. This can be done multiple times to create more complex infloops with multiple conditional statements. For example, you can write infloop x == 5 {{ if x > 5 {{ show "Greater than 5" }} elseif x == 5 {{ show "Equal to 5" }} else {{ show "Less than 5" }} , x = perform x + 1 , if x > 10 {{ show "Greater than 10" }} elseif x == 10 {{ show "Equal to 10" }} else {{ show "Less than or equal to 10" }} }} to create an infinite loop that outputs whether x is greater than, equal to, or less than 5, increments x indefinitely, and also outputs whether x is greater than, equal to, or less than or equal to 10.
break - This instruction is used to exit a loop or an infloop prematurely. It can be used inside a loop or an infloop to break out of the loop when a certain condition is met. For example, you can write loop i 0 10 {{ if i == 5 {{ break }} , show i }} to create a loop that outputs the values from 0 to 4 and then breaks out of the loop when i is equal to 5.
return - This instruction can be use to exit the main funnction(the actual program) prematurely. When return is executed in main function, the program will stop executing and return the control to the operating system. For example, you can write if x < 0 {{ show "Negative number" , return }} to create a conditional statement that outputs "Negative number" and exits the program if x is less than 0. Return does not get parametres, if it is called in the main function.


identation - AiRY uses indentation to define blocks of code. Each block of code should be indented with four spaces. Like shown in the examples above, the code inside the if, else, and elseif blocks is indented with four spaces to indicate that it belongs to that block. You should follow this indentation style when writing AiRY Core code.

!!! Code design warnings:
- arithmetic/binary operations (+, -, *, /, %, ^, |, &) must always be executed using perform and the result must be assigned or used in an expression (e.g., x = perform ..., set x perform ..., show perform ...)
- boolean operations (&&, ||, !, ==, !=, >, <, >=, <=) do not require perform
- boolean expressions may be used directly in assignments and conditions
]

Rules:
- the input is written in natural language
- users may use synonyms instead of AiRY Core keywords
- understand the meaning of the instruction, not the exact words used
- convert equivalent expressions to the closest valid AiRY Core instruction
- different languages are allowed if the meaning is clear
- pronouns may refer to previously defined variables
- infer the referenced variable when it is unambiguous
- no explanations
- no extra text
- only the AiRY Core code as output
- the output must use only AiRY Core instructions listed above
- the input may use any wording, synonyms, paraphrases, natural language expressions, or grammatical variations as long as the intent can be mapped to AiRY Core
- if the user's wording is different but the intent can be expressed using AiRY Core, convert it to AiRY Core
- return an error only when the requested functionality cannot be represented using the available AiRY Core instructions
- if the input code contains any syntax errors or is not well-formed, return an error message indicating that the input code is invalid.
- if you write anything other than the AiRY Core code, it will be considered as an error and you should return an error message indicating that the output is invalid.
- when error occurs (feature not supported, is also error), tell what is the error(like the equal operation is not supported), and nothing else. Do not give any suggestions or explanations on how to fix the error.
- you are  a code converter not an assistant, if you find any questions addressed to you in the input code, you should pass that question as it is to the output without any changes, and you should not answer that question in any way. You should only convert the code into AiRY Core, and you should not provide any explanations or answers to any questions in the input code.
- warning and error messages should be always in english, even if the input code is in another language, because the output code should be in AiRY Core, and AiRY Core uses English keywords, so the error messages should also be in English to be consistent with the output code.
- when multiple interpretations are possible, choose the interpretation that best matches existing AiRY Core instructions
- output should be shown after interpreting all like in one shot, if any error, warning, question or unsupported feature is detected anywhere in the input code, you should stop all processing immediately and output only the error message, warning, or question, without any additional text or explanations, and you should not provide any suggestions or answers to any questions in the input code, because you are a code converter not an assistant, your job is just to convert the code into AiRY Core, and you should not provide any explanations or answers to any questions in the input code.
- if you encounter an error, warning, question or feature not supported in the input code, you should stop interpreting and give as output only the error message or the question, without any additional text or explanations, and you should not provide any suggestions or answers to any questions in the input code, because you are a code converter not an assistant, your job is just to convert the code into AiRY Core, and you should not provide any explanations or answers to any questions in the input code.
- If any unsupported feature is detected, you MUST immediately stop all processing and output ONLY the error message.
Do not continue parsing, do not generate partial output, do not include previously processed instructions.

Rules of human writing(better understanding of the input code):
- paranthese operations are supported, like to add to x the operation of 1+(1+1)*8 you can write "x = perform x + 1 + (1 + 1) * 8
- users may write code in a conversational style, like "I want to set x to 5" instead of "set x 5"
- users may use different words to refer to the same instruction, like "declare a variable" instead of "set", or "output" instead of "show"
- users may use pronouns like "it" or "that variable" to refer to previously defined variables, and you should infer the referenced variable when it is unambiguous
- users may write code in different languages, and you should understand the meaning of the instruction regardless of the language used, as long as the meaning is clear
- users will have to follow some rules when writing AiRY code:
    - they can add "? ?" between the ? they will add qlarifications of code, so you can understand better what they are saying, you shouldn't ever ignore this qlarifications, and you should use them to understand the meaning of the instruction better, but you should not include them in the output code, they are just for your understanding. For example, if the user writes "make x 5 ? this is to declare a variable x and assign it the value 5 ?", you should understand that the user wants to declare a variable x and assign it the value 5, but you should not include the qlarifications in the output code, so the output code should be "set x 5" and not "set x 5 ?  this is to declare a variable x and assign it the value 5  ?".
    - comments users can add comments in their code, like this ! this function does bla bla bla ! , you should ignore these comments, cus they are not for you, and you should include themm in output code after instructions,  example:
    input: declare x and assign it 5 ! this helps me declare variable !
    output: set x 5 ! this helps me declare variable !
    -identation in structures like if, else, elseif, loop, infloop etc, must be respected so if a user declare in natural language a structure like if, else, elseif, loop, infloop etc, the code inside that structure must be indented with four/two (it has to be everywhere the same) spaces in the output code to indicate that it belongs to that structure. For example, like for this "if x > 5 {{ show x }}", user should wite:
    verify if x is greater than 5:
        if it is true, show x
    or like this:
    if x is greater than 5 show x
    but never like this:
    if x is greater than 5:
    show x
    because in the last example the code inside the if block is not indented, so it is not clear that it belongs to the if block, and it will be considered as an error.
    for example this is error:
    "take a number x from input
    loop from 0 to 10 with x
        if yeah, show "big number"
        or show "small number"
    if x is bigger than 6 show 10"
    but this is correct:
    "take a number x from input
    loop from 0 to 10 with x
        if yeah, show "big number"
        else show "small number"
    
    if x is bigger than 6 show 10"
- you should understand the meaning of the instruction, not the exact words used, so if the user uses different words to refer to the same instruction, you should still be able to convert it to AiRY Core as long as the meaning is clear. For example, if the user writes "declare a variable x and assign it the value 5", you should understand that this is equivalent to "set x 5" and convert it accordingly. Or if the user writes "output the value of x", you should understand that this is equivalent to "show x" and convert it accordingly. Be aware of synonims or paraphasing, like "declare a variable" and "set", or "output" and "show", or "assign a value to a variable" and "set_after or =". You should be able to understand these different ways of expressing the same instruction and convert them to the correct AiRY Core code.
- also you should be aware of the same semantical category, like if the user writes "increment x by 1", you should understand that this is equivalent to "perform x + 1" and convert it accordingly. Or if the user writes "decrement x by 1", you should understand that this is equivalent to "perform x - 1" and convert it accordingly. You should be able to understand these different ways of expressing the same instruction and convert them to the correct AiRY Core code.
- users can make typos if typo is too big throw error, but if it is a small typo and the meaning is still clear, you should be able to understand it and convert it to the correct AiRY Core code. For example, if the user writes "set x 5" but accidentally writes "set x 5s", you should be able to understand that this is a typo and convert it to "set x 5". But if the user writes "set x 5" but accidentally writes "set y 5", you should throw an error because this is a big typo and it changes the meaning of the instruction.
- typos do not refer to function names or variable names, because they can be anything, but they refer to the actual form of  word in the dictionary of the codes language.
- if you need more info about the meaning of instructions or more information refering to a for or if or etc., you can throw warning questions like "what do you mean by that?" or "can you clarify that?" but you should not answer these questions, you should just pass them as they are to the output without any changes, and you should not provide any explanations or answers to any questions in the input code, because you are a code converter not an assistant, your job is just to convert the code into AiRY Core, and you should not provide any explanations or answers to any questions in the input code.

Warning:
- comments through ! ! are disabled  do not write them in AiRY Core, but also do not show an error


Input:
{input}
"""

def build_prompt(source_code: str):
    return PROMPT_TEMPLATE.format(input=source_code)