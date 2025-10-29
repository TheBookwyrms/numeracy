from math_stuff.node_class import *
from math_stuff.derive_tree import derive_tree

class SequencenException(Exception):
    def __init__(self, *args):
        super().__init__(*args)

class Sequence:
    def __init__(self, general_term:MathNode, var_of_sequence_iteration:str):
        self.general_term = general_term
        self.n = var_of_sequence_iteration

    def get_n_terms(self, last_term:int=10, first_term:int=0, step:int=1, specific_terms=None):
        iteration_terms = [first_term, last_term, step]
        all_terms_int = all([True if type(term)==int else False for term in iteration_terms])
        if not all_terms_int:
            raise SequencenException('first term, last term, and step must be integers')
        terms = []
        if specific_terms == None:
            for i in range(*iteration_terms):
                terms.append(self.general_term.sub_variables({self.n:i}))            
        else:
            for i in specific_terms:
                if type(i) != int:
                    raise Sequence('nth term must be an integer')
                terms.append(self.general_term.sub_variables({self.n:i}))            

        return tuple(terms)

        def find_limit(self):
            pass




def run_3():

    n = MathNode(Subtypes.VAR, "n")
    k = MathNode(Subtypes.VAR, "n")
    b = MathNode(Subtypes.VAR, "b")
    two = MathNode(Subtypes.NUMBER, 2)
    an = tan(two*n*b)
    sq = Sequence(an, n.value)
    terms = sq.get_n_terms(5)
    print(tuple([str(t) for t in terms]))
    #print(an.simplify(), an.simplify_multiple_multiplications())

    sn = (n+n+n+n+n+n+n+n)
    #sn2 = sn.simplify()
    #print(sn)
    #print(sn2)
    #print(tan(2+2), tan(2+2).simplify())

    m = ((n-b+n-two+two)-n-n+two-two+(two-b-b+n+n)-(b-b+two)+two+two-(b+b+b)-n+n-two)-two-two-n
    #m = two-two-n
    #m = ((n-b+n-two+two)-n-n+two-two+(two-b-b+n+n)-(b-b+two)+two+two-(b+b+b)-n+n-two)
    #m = (b-n) - (b-two) - (two - two)
    ms = m.compress_multiple_subtractions()
    print(m)
    print(ms)
    ms2 = ms.simplify_multiple_additions()
    print(ms2)
    ms3 = ms2.compress_add_subtract()
    print("ms3", ms3)
    ms4 = ms3.simplify_multiple_additions()
    print("ms4", ms4)
    ms5 = ms4.compress_add_subtract()
    print("ms5", ms5)
    ms6 = ms5.simplify_multiple_additions()
    print("ms6", ms6)

    print()
    mss = m.simplify()
    print("mss ", mss)
    print(derive_tree(mss))
    #k = Node(subtypes.NUMBER, 2)
    #m = Node(k.subtype, k.value)
    #print(m == k, m is k)
    #k.value = 4
    #print(m == k, m is k, m.value)