# disassembler for 3-bit computer
# tail -1 input | cut -d: -f2- | python as.py

def combo_operand(operand):
    if operand <= 3:
        return str(operand)
    elif operand == 4:
        return 'A'
    elif operand == 5:
        return 'B'
    elif operand == 6:
        return 'C'

def literal_operand(operand):
    return str(operand)

# program = [2,4,1,6,7,5,4,4,1,7,0,3,5,5,3,0]
program = [int(x) for x in input().split(',')]

instructions = [
    ('adv', combo_operand),
    ('bxl', literal_operand),
    ('bst', combo_operand),
    ('jnz', literal_operand),
    ('bxc', literal_operand),
    ('out', combo_operand),
    ('bdv', combo_operand),
    ('cdv', combo_operand)
]

for i in range(0, len(program), 2):
    opcode = program[i]
    operand = program[i + 1]
    print(instructions[opcode][0], instructions[opcode][1](operand))
