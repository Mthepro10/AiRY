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
operations - AiRY supports basic arithmetic operations like addition (+), subtraction (-), multiplication (*), and division (/). You can use these operations to perform calculations on variables and values. For example, you can write set x 5 + 3 to assign the result of the addition to variable x.
perform an operation - This instruction is used to perform an operation on variables; It takes a variable name, an operator, and another variable or value as arguments and updates the variable with the result of the operation. For example, you can write perform x + 2 to add 2 to the current value of x and update x with the new value.
bitwise operations - AiRY also supports bitwise operations like AND (&), OR (|), XOR (^) , and NOT (~). You can use these operations to perform bitwise calculations on variables and values. For example, you can write set x 5 & 3 to assign the result of the bitwise AND operation to variable x.
performing a bitwise operation - This instruction is used to perform a bitwise operation on variables; It takes a variable name, a bitwise operator, and another variable or value as arguments and updates the variable with the result of the bitwise operation. For example, you can write perform x & 2 to perform a bitwise AND operation between the current value of x and 2, and update x with the new value.
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
set_after or = - Unlike set x y wich is to declare variables and (optional) assign values to them, set_after or = is used to assign a new value to an already declared variable. It takes a variable name, an operator, and another variable or value as arguments and updates the variable with the new value. For example, you can write x = 5 to assign the value 5 to variable x, or you can write x = y + 2 to assign the result of the addition of y and 2 to variable x. In actual code just = will be used instead of set_after, but I am mentioning set_after here to make it clear that this is a different instruction from set, and it is used for a different purpose.

identation - AiRY uses indentation to define blocks of code. Each block of code should be indented with four spaces. Like shown in the examples above, the code inside the if, else, and elseif blocks is indented with four spaces to indicate that it belongs to that block. You should follow this indentation style when writing AiRY Core code.
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
- use only the instructions listed above to convert the given code into AiRY Core
- if the user's wording is different but the intent can be expressed using AiRY Core, convert it to AiRY Core
- return an error only when the requested functionality cannot be represented using the available AiRY Core instructions
- if the input code contains any syntax errors or is not well-formed, return an error message indicating that the input code is invalid.
- if you write anything other than the AiRY Core code, it will be considered as an error and you should return an error message indicating that the output is invalid.
- when error occurs (feature not supported, is also error), tell what is the error(like the equal operation is not supported), and nothing else. Do not give any suggestions or explanations on how to fix the error.

Input:
{input}
"""

def build_prompt(source_code: str):
    return PROMPT_TEMPLATE.format(input=source_code)