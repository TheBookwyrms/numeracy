from math_stuff.node_class import *

class DifferentiabilityException(Exception):
    def __init__(self, *args):
        super().__init__(*args)


def derive_tree(tree:MathNode):

    if tree.subtype == Subtypes.NULL:
        return MathNode(Subtypes.NULL, Null.NONE)
    elif tree.subtype == Subtypes.CONSTANT:
        return MathNode(Subtypes.NULL, Null.NONE)
    elif tree.subtype == Subtypes.NUMBER:
        return MathNode(Subtypes.NULL, Null.NONE)
    elif tree.subtype == Subtypes.VAR:
        return MathNode(Subtypes.VAR, tree.value+"'")
    elif tree.subtype == Subtypes.OPERATION:
        match tree.value:
            case Operations.ADD:
                derived_children = []
                for child in tree.children:
                    derived_children.append(derive_tree(child))

                return MathNode(Subtypes.OPERATION, Operations.ADD, children=tuple(derived_children))
            
            case Operations.MULTIPLY:
                if len(tree.children)==2:
                    c0 = tree.children[0]
                    c0_derived = derive_tree(c0)
                    c1 = tree.children[1]
                    c1_derived = derive_tree(c1)
                    left = MathNode(Subtypes.OPERATION, Operations.MULTIPLY, children=(c0_derived, c1))
                    right = MathNode(Subtypes.OPERATION, Operations.MULTIPLY, children=(c0, c1_derived))
                    total = MathNode(Subtypes.OPERATION, Operations.ADD, children=(left, right))
                    return total
                elif len(tree.children)>2:
                    c_left = tree.children[0]
                    c_left_derived = derive_tree(c_left)
                    c_else = MathNode(Subtypes.OPERATION, Operations.MULTIPLY, children=tuple(tree.children[1:]))
                    c_else_derived = derive_tree(c_else)
                    left = MathNode(Subtypes.OPERATION, Operations.MULTIPLY, children=(c_left_derived, c_else))
                    right = MathNode(Subtypes.OPERATION, Operations.MULTIPLY, children=(c_left, c_else_derived))
                    total = MathNode(Subtypes.OPERATION, Operations.ADD, children=(left, right))
                    return total
                else:
                    raise ChildrenException(f'multiplication should not have {len(tree.children)} children')
            
            case Operations.SUBTRACT:
                c0_derived = derive_tree(tree.children[0])
                c1_derived = derive_tree(tree.children[1])
                return MathNode(Subtypes.OPERATION, Operations.SUBTRACT, children=(c0_derived, c1_derived))

            case Operations.DIVIDE:
                c0_derived = derive_tree(tree.children[0])
                c1_derived = derive_tree(tree.children[1])
                return MathNode(Subtypes.OPERATION, Operations.DIVIDE, children=(c0_derived, c1_derived))

            case Operations.FLOOR_DIVISION:
                raise DifferentiabilityException(f"operation {tree.value} is not differentiable")
            
            case Operations.MODULO_DIVISION:
                raise DifferentiabilityException(f"operation {tree.value} is not differentiable")
            
            case Operations.POWER:
                f = tree
                g = tree.children[0]
                g_prime = derive_tree(g)
                h = tree.children[1]
                h_prime = derive_tree(h)

                '''
                f = g**h
                ln(f) = h*ln(g)
                f'/f = h'*ln(g) + h*g'/g
                f'(x) = f(x) * (h'(x) * ln(g(x)) + h(x) * g'(x) / g(x))
                '''

                f_prime = f * ((h_prime * ln(g)) + (h * g_prime / g))

                return f_prime
            
            case Operations.LOG10:
                g = tree.children[0]
                g_prime = derive_tree(g)
                ten = MathNode(Subtypes.NUMBER, 10)

                return g_prime / (g * ln(ten))
            
            case Operations.LN:
                g = tree.children[0]
                g_prime = derive_tree(g)

                return g_prime / g
            
            case Operations.SIN:
                g = tree.children[0]
                g_prime = derive_tree(g)

                return cos(g)*g_prime

            case Operations.COS:
                g = tree.children[0]
                g_prime = derive_tree(g)
                negative_one = MathNode(Subtypes.NUMBER, -1)

                return negative_one*sin(g)*g_prime
            
            case Operations.TAN:
                g = tree.children[0]
                g_prime = derive_tree(g)
                two = MathNode(Subtypes.NUMBER, 2)

                return (sec(g)**two) * g_prime
            
            case Operations.CSC:
                g = tree.children[0]
                g_prime = derive_tree(g)
                negative_one = MathNode(Subtypes.NUMBER, -1)

                return negative_one * csc(g) * cot(g) * g_prime
            
            case Operations.SEC:
                g = tree.children[0]
                g_prime = derive_tree(g)

                return sec(g) * tan(g) * g_prime
            
            case Operations.COT:
                g = tree.children[0]
                g_prime = derive_tree(g)
                two = MathNode(Subtypes.NUMBER, 2)
                negative_one = MathNode(Subtypes.NUMBER, -1)

                return negative_one * (csc(g)**two) * g_prime
            
            case Operations.ARCSIN:
                g = tree.children[0]
                g_prime = derive_tree(g)
                one = MathNode(Subtypes.NUMBER, 1)
                half = MathNode(Subtypes.NUMBER, 0.5)
                two = MathNode(Subtypes.NUMBER, 2)

                return g_prime / ((one - g**two)**half)
            
            case Operations.ARCCOS:
                g = tree.children[0]
                g_prime = derive_tree(g)
                one = MathNode(Subtypes.NUMBER, 1)
                half = MathNode(Subtypes.NUMBER, 0.5)
                two = MathNode(Subtypes.NUMBER, 2)
                negative_one = MathNode(Subtypes.NUMBER, -1)

                return negative_one * g_prime / ((one - g**two)**half)
            
            case Operations.ARCTAN:
                g = tree.children[0]
                g_prime = derive_tree(g)
                one = MathNode(Subtypes.NUMBER, 1)
                two = MathNode(Subtypes.NUMBER, 2)

                return g_prime / (one + g**two)
            
            case Operations.ARCCSC:
                g = tree.children[0]
                g_prime = derive_tree(g)
                one = MathNode(Subtypes.NUMBER, 1)
                two = MathNode(Subtypes.NUMBER, 2)
                negative_one = MathNode(Subtypes.NUMBER, -1)

                return negative_one * g_prime / (abs(g) * (g**two - 1)**half)
            
            case Operations.ARCSEC:
                g = tree.children[0]
                g_prime = derive_tree(g)
                one = MathNode(Subtypes.NUMBER, 1)
                two = MathNode(Subtypes.NUMBER, 2)

                return g_prime / (abs(g) * (g**two - 1)**half)
            
            case Operations.ARCCOT:
                g = tree.children[0]
                g_prime = derive_tree(g)
                one = MathNode(Subtypes.NUMBER, 1)
                two = MathNode(Subtypes.NUMBER, 2)
                negative_one = MathNode(Subtypes.NUMBER, -1)

                return negative_one * g_prime / (one + g**two)
            
            case Operations.ABS:
                raise DifferentiabilityException(f"operation {tree.value} is not differentiable")
            
            case Operations.FLOOR:
                raise DifferentiabilityException(f"operation {tree.value} is not differentiable")
            
            case Operations.CEIL:
                raise DifferentiabilityException(f"operation {tree.value} is not differentiable")
    else:
        raise SubtypeException(f"subtype {tree.subtype} is not a supported Node subtype")
    

def run_2():

    five = MathNode(Subtypes.NUMBER, 5)
    x = MathNode(Subtypes.VAR, "x")
    y = MathNode(Subtypes.VAR, "y")
    z = MathNode(Subtypes.VAR, "z")
    a = MathNode(Subtypes.VAR, "a")
    b = MathNode(Subtypes.VAR, "b")
    c = MathNode(Subtypes.VAR, "c")
    test = five+x+14
    m_test = x*y*z*a*b*c
    a2 = x+y+z+a+b+c

    a3 = ((x**y) * (a*b))
    a4 = tan(a3)
    #print("   ", a3)
    #print(a4)
    #print("                           ", derive_tree(a3))
    #print(derive_tree(a4))

    #print(a2.simplify_multiple_additions())
    #mt2 = m_test.simplify_multiple_multiplications()
    #mt4 = derive_tree(mt2)

    subs_dict = {
        "a":5,
    }
    print(a4)
    print(a4.sub_variables(subs_dict))

    #print(derive_tree(five), derive_tree(x))
    #print(derive_tree(test).simplify_multiple_additions())
    #print()
    #print(m_test)
    #print(mt2)
    #print(mt4)


if __name__ == "__main__":
    run_2()