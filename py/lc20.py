def validParenthesis(s):
    stack = []

    for c in s:
        if c == '(' or c == '{' or c =='[':
            stack.append(c)
        elif c == ')':
            if not stack or stack.pop() != '(':
                return False
        elif c == '}':
            if not stack or stack.pop() != '{':
                return False
        elif c == ']':
            if not stack or stack.pop() != '[':
                return False
        else:
            print(f"Invalid character: {c}")
            return False

    return len(stack) == 0
