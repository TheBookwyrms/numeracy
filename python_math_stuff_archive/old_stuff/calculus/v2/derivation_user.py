from math_stuff.calculus.v2.tree_builder_2 import *
from math_stuff.calculus.v2.derive_operation import *
from math_stuff.calculus.v2.simplifications_compressions import *
from math_stuff.calculus.v2.substitute_variables import *
from math_stuff.calculus.v2.calculate_value import *



def run():
    o = node(operations.const, 1)
    ono = node(operations.const, 1)
    t = node(operations.const, 2)
    th = node(operations.const, 3)
    f = node(operations.const, 4)
    fi = node(operations.const, 5)
    _s = node(operations.const, -7)
    x = node(operations.var, "x")
    x_p = node(operations.var, "x'")
    y = node(operations.var, "y")

    ch = o/(th*(x**th))
    ch = (th*(x**t))/(x*x*x*t)
    #print(ch)

    ch=asin(cos(x)+cos(y))
    ch = cos(sin(x*y))

    
    #ch = ((th*(x**t))/(x*x*x*t))/((x**th)/((th*(x**t))/(x*x*x*t)))
    #ch = (x+x_p)*t
    #ch = th*(x**t)
    #ch = x**t+x**t+x**t-x**t-x**t+ln(t)+ln(t)+ln(t)
    #ch = sec(csc(x**x))
    #ch = ln(t)
    #ch = x**(ln(x))
    #ch = o*t+(x**t)
    #ch = (x*x*x*t)
    #ch = ch+fi
    t = derive_operation(ch, with_respect_to="all")
    print()
    print(f'{colourer('32')}original expression{colourer(0)}:       {ch}') # remove 3 spaces later
    t_final = all_simplifications_and_compressions(t)
    print(f'{colourer('33')}derived expression{colourer(0)}:     {t}')
    print(f'{colourer('34')}simplified expression{colourer(0)}:  {t_final}')

    print()
    #substitutions = {
    #    "x" : 15,
    #    "y" : -2.438,
    #    "y'" : 0,
    #    "x'" : "y'",
    #}
    #print(f'{substitute_var_as(t_final, substitutions)}')
    print()


    print(sec(f)+csc(f), "=", calculate_value_of(sec(f)+csc(f)))


def ben():
    e = node(operations.const, 2.71828)
    one = node(operations.const, 1)
    x = node(operations.var, "x")
    ten = node(operations.const, 10)
    half = node(operations.const, 0.5)

    f_of_x = (sin(x)+one)**x

    #f_of_x = e**(sin(atan(x**half)))
    #print(f_of_x)

    f_prime = derive_operation(f_of_x, with_respect_to="x")
    #print(f_prime)
    s_f = all_simplifications_and_compressions(f_prime)
    #print(s_f)

    f_p_of_10 = substitute_var_as(s_f, {"x": 3*3.14159/2, "x'":1})

    print(f_p_of_10)
