program = [2,4,1,6,7,5,4,4,1,7,0,3,5,5,3,0]

def bt(program, A):
    if len(program) == 0:
        return A
    for a in range(0, 8):
        A = (A << 3) | a
        B = a ^ 0x6
        C = A >> B
        B = (B ^ C) ^ 0x7
        if B & 0x7 == program[0]:
            ans = bt(program[1:], A)
            if ans != None:
                return ans
        A >>= 3
    return None

print('A =', bt(program[::-1], 0))
