class Subtypes:
    NULL = 0
    OPERATION = 1
    VAR = 2
    NUMBER = 3
    CONSTANT = 4
    INFINITY = 5


class Constant:
    pass

class Infinity:
    pass

class Null:
    NONE = None



class Operations:
    ADD = 1
    SUBTRACT = 2
    MULTIPLY = 3
    DIVIDE = 4
    FLOOR_DIVISION = 5
    MODULO_DIVISION = 6

    POWER = 11
    LOG10 = 12
    LN = 13

    SIN = 21
    COS = 22
    TAN = 23
    CSC = 24
    SEC = 25
    COT = 26

    ARCSIN = 31
    ARCCOS = 32
    ARCTAN = 33
    ARCCSC = 34
    ARCSEC = 35
    ARCCOT = 36

    ABS = 41
    FLOOR = 42
    CEIL = 43


    to_symbol = {
        1  : "+",
        2  : "-",
        3  : "*",
        4  : "/",
        5  : "//",
        6  : "%",

        11 : "**",
        12 : "log10",
        13 : "ln",

        21 : "sin",
        22 : "cos",
        23 : "tan",
        24 : "csc",
        25 : "sec",
        26 : "cot",

        31 : "arcsin",
        32 : "arccos",
        33 : "arctan",
        34 : "arccsc",
        35 : "arcsec",
        36 : "arccot",

        41 : "abs",
        42 : "floor",
        43 : "ceil",
    }




class SubtypeException(Exception):
    def __init__(self, *args):
        super().__init__(*args)

class ChildrenException(Exception):
    def __init__(self, *args):
        super().__init__(*args)


class MathNode:
    def __init__(self, subtype:Subtypes, value, children=()):
        self.subtype = subtype
        self.value = value
        self.children = children

    def __str__(self):        
        if self.subtype == Subtypes.NULL:
            return f'{self.value}'
        elif self.subtype == Subtypes.OPERATION:

            if self.value == Operations.MULTIPLY:

                items_to_add = [f'{str(child)}' for child in self.children]
                
                return "("+Operations.to_symbol[self.value].join(items_to_add)+")"
            
            elif self.value == Operations.ADD:

                items_to_add = [f'{str(child)}' for child in self.children]
                
                return "("+Operations.to_symbol[self.value].join(items_to_add)+")"
                
            else:
                if len(self.children) == 1:
                    return f'{Operations.to_symbol[self.value]}({str(self.children[0])})'
                elif len(self.children) == 2:
                    return f'({str(self.children[0])}{Operations.to_symbol[self.value]}{str(self.children[1])})'
                else:
                    raise ChildrenException(f'''operation type {self.value}
                                            should not have {len(self.children)} children''')
            # TODO
        elif self.subtype == Subtypes.VAR:
            return f'{self.value}'
        elif self.subtype == Subtypes.NUMBER:
            return f'{self.value}'
        else:
            raise SubtypeException(f"no subtype {self.subtype} exists")
    
    def __add__(self, *siblings):
        return MathNode(Subtypes.OPERATION, Operations.ADD, children=as_Node(self, *siblings))
    
    def __mul__(self, *siblings):
        return MathNode(Subtypes.OPERATION, Operations.MULTIPLY, children=as_Node(self, *siblings))
    
    def __sub__(self, sibling):
        return MathNode(Subtypes.OPERATION, Operations.SUBTRACT, children=as_Node(self, sibling))

    def __truediv__(self, sibling):
        return MathNode(Subtypes.OPERATION, Operations.DIVIDE, children=as_Node(self, sibling))

    def __floordiv__(self, sibling):
        return MathNode(Subtypes.OPERATION, Operations.FLOOR_DIVISION, children=as_Node(self, sibling))

    def __mod__(self, sibling):
        return MathNode(Subtypes.OPERATION, Operations.MODULO_DIVISION, children=as_Node(self, sibling))

    def __pow__(self, sibling):
        return MathNode(Subtypes.OPERATION, Operations.POWER, children=as_Node(self, sibling))
    
    def __abs__(self):
        return MathNode(Subtypes.OPERATION, Operations.ABS, children=as_Node(self))
    
    
    def simplify_multiple_additions(self):
        new_children = []
        for child in self.children:
            new_children.append(child.simplify_multiple_additions())

        if not ((self.subtype == Subtypes.OPERATION) and (self.value == Operations.ADD)):
            return MathNode(self.subtype, self.value, children=tuple(new_children))
        else:

            is_add = lambda x : ((x.subtype == Subtypes.OPERATION) and (x.value == Operations.ADD))
            new_is_add = []
            for child in new_children:
                new_is_add.append(is_add(child))

            children_for_return = []
            for child, add in zip(new_children, new_is_add):
                if add:
                    children_for_return.extend([*child.children])
                else:
                    children_for_return.append(child)

            return MathNode(Subtypes.OPERATION, Operations.ADD, children=tuple(children_for_return))
    
    def simplify_multiple_multiplications(self):
        new_children = []
        for child in self.children:
            new_children.append(child.simplify_multiple_multiplications())

        if not ((self.subtype == Subtypes.OPERATION) and (self.value == Operations.MULTIPLY)):
            return MathNode(self.subtype, self.value, children=tuple(new_children))
        else:
            #print(new_children[0])
            c0 = new_children[0]
            #print(c0)
            c1 = new_children[1]

            c0_mult = ((c0.subtype == Subtypes.OPERATION) and (c0.value == Operations.MULTIPLY))
            c1_mult = ((c1.subtype == Subtypes.OPERATION) and (c1.value == Operations.MULTIPLY))

            if c0_mult and c1_mult:
                k = MathNode(Subtypes.OPERATION, Operations.MULTIPLY, children=(*c0.children, *c1.children))
                return k
            elif c0_mult and (not c1_mult):
                k = MathNode(Subtypes.OPERATION, Operations.MULTIPLY, children=(*c0.children, c1))
                return k
            elif (not c0_mult) and c1_mult:
                k = MathNode(Subtypes.OPERATION, Operations.MULTIPLY, children=(c0, *c1.children))
                return k
            else:
                k = MathNode(Subtypes.OPERATION, Operations.MULTIPLY, children=(c0, c1))
                return k
            
    def __eq__(self, sibling):
        if type(sibling) != MathNode:
            return False

        if self.subtype == sibling.subtype:
            if self.value == sibling.value:
                if len(self.children) == len(sibling.children):
                    if len(self.children) == 0:
                    #if len(self.children):
                        return True
                    else:
                        c0_in_c1 = []
                        for c0 in self.children:
                            for c1 in sibling.children:
                                if c0==c1:
                                    c0_in_c1.append(True)
                                    c0=[None,]
                            if c0 != [None,]:
                                c0_in_c1.append(False)
                        if all(c0_in_c1):
                            return True
                        
        return False

            
    def compress_multiple_additions(self):
        new_children = []
        for child in self.children:
            new_children.append(child.compress_multiple_additions())

        if not ((self.subtype == Subtypes.OPERATION) and (self.value == Operations.ADD)):
            return MathNode(self.subtype, self.value, children=tuple(new_children))
        else:
            unique_terms = []
            num_appearances = []
            for child in new_children:
                child_in_terms = False
                for item in unique_terms:
                    if child == item:
                        child_in_terms = True
                if child_in_terms:
                    index = ""
                    for i, j in enumerate(unique_terms):
                        if j == child:
                            index = i
                    num_appearances[index] += 1
                else:
                    unique_terms.append(child)
                    num_appearances.append(1)
            
            final_terms = []
            for term, i in zip(unique_terms, num_appearances):
                if i != 1:
                    num = MathNode(Subtypes.NUMBER, i)
                    compressed = MathNode(Subtypes.OPERATION, Operations.MULTIPLY, children=(num, term))
                else:
                    compressed = term
                final_terms.append(compressed)
            
            return MathNode(Subtypes.OPERATION, Operations.ADD, children=(tuple(final_terms)))
        

    def compress_multiple_subtractions(self):
        new_children = []
        for child in self.children:
            new_children.append(child.compress_multiple_subtractions())

        if not ((self.subtype == Subtypes.OPERATION) and (self.value == Operations.SUBTRACT)):
            return MathNode(self.subtype, self.value, children=tuple(new_children))
        else:
            c0 = new_children[0]
            c1 = new_children[1]

            c0_is_sub = ((c0.subtype == Subtypes.OPERATION) and (c0.value == Operations.SUBTRACT))
            c1_is_sub = ((c1.subtype == Subtypes.OPERATION) and (c1.value == Operations.SUBTRACT))

            if c0_is_sub and c1_is_sub:
                # (a-b) - (c-d) = a-b-c+d = (a+d)-(c+d)
                a = c0.children[0]
                b = c0.children[1]
                c = c1.children[0]
                d = c1.children[1]
                left = MathNode(Subtypes.OPERATION, Operations.ADD, children=(a, d))
                right = MathNode(Subtypes.OPERATION, Operations.ADD, children=(b, c))
                return MathNode(Subtypes.OPERATION, Operations.SUBTRACT, children=(left, right))

            elif c0_is_sub and (not c1_is_sub):
                # (a-b) - (c) = a-b-c = a - (b+c)
                a = c0.children[0]
                b = c0.children[1]
                c = c1
                left = a
                right = MathNode(Subtypes.OPERATION, Operations.ADD, children=(b, c))
                return MathNode(Subtypes.OPERATION, Operations.SUBTRACT, children=(left, right))

            elif (not c0_is_sub) and c1_is_sub:
                # (a) - (b-c) = a-b+c = (a+c) - b
                a = c0
                b = c1.children[0]
                c = c1.children[1]
                left = MathNode(Subtypes.OPERATION, Operations.ADD, children=(a, c))
                right = b
                return MathNode(Subtypes.OPERATION, Operations.SUBTRACT, children=(left, right))

            else:
                # (a) - (b) = a-b
                return MathNode(Subtypes.OPERATION, Operations.SUBTRACT, children=tuple(new_children))
            

    def compress_add_subtract(self):
        new_children = []
        for child in self.children:
            new_children.append(child.compress_add_subtract())

        is_add = ((self.subtype == Subtypes.OPERATION) and (self.value == Operations.ADD))
        is_sub = lambda x : ((x.subtype == Subtypes.OPERATION) and (x.value == Operations.SUBTRACT))

        if not is_add:
            return MathNode(self.subtype, self.value, children=tuple(new_children))
        else:
            # for :
            #    (-) + (-)
            #    (+) + (-)
            #    (-) + (+)
            subtractions = []
            others = []
            for child in new_children:
                if is_sub(child) :
                    subtractions.append(child)
                else:
                    others.append(child)

            if subtractions != []:
                sub_first_terms = []
                sub_second_terms = []
                for sub in subtractions:
                    sub_first_terms.append(sub.children[0])
                    sub_second_terms.append(sub.children[1])


                additions = others + sub_first_terms

                left = MathNode(Subtypes.OPERATION, Operations.ADD, children=tuple(additions))

                if len(sub_second_terms) > 1:
                    right = MathNode(Subtypes.OPERATION, Operations.ADD, children=tuple(sub_second_terms))
                else:
                    right = sub_second_terms[0]

                return MathNode(Subtypes.OPERATION, Operations.SUBTRACT, children=(left, right))
            else:
                return MathNode(Subtypes.OPERATION, Operations.ADD, children=tuple(others))
            
    def remove_nones(self):
        new_children = []
        for child in new_children:
            new_children.append(child.remove_nones())

        if self.subtype == Subtypes.CONSTANT:
            return MathNode(self.subtype, self.value, children=tuple(new_children))
        elif self.subtype == Subtypes.INFINITY:
            return MathNode(self.subtype, self.value, children=tuple(new_children))
        elif self.subtype == Subtypes.NULL:
            return
        elif self.subtype == Subtypes.NUMBER:
            return MathNode(self.subtype, self.value, children=tuple(new_children))
        elif self.subtype == Subtypes.VAR:
            return MathNode(self.subtype, self.value, children=tuple(new_children))
        elif self.subtype == Subtypes.OPERATION:
            pass
            
    def simplify(self):

        original = 0
        n_times = 0

        print("org", self)


        while original != self:

            original = MathNode(self.subtype, self.value, self.children)
            
            self = self.compress_multiple_subtractions()

            self = self.simplify_multiple_additions() # puts continuous additions into a single parent
            self = self.compress_add_subtract()
        
        print("a-s", self)
        self = self.compress_multiple_additions() # puts duplicates into multiplications
        self = self.simplify_multiple_multiplications() # puts multiple multiplications into a single parent
            #n_times += 1
            #print(original is self)
        #print("n_times", n_times)

        return self
            
    def sub_variables(self, substitutions:dict):
        new_children = []
        for child in self.children:
            new_children.append(child.sub_variables(substitutions))
        
        if self.subtype == Subtypes.VAR:
            if self.value in substitutions:
                new_value = substitutions[self.value]

                if type(new_value) == str:
                    return MathNode(Subtypes.VAR, new_value)
                elif type(new_value) == Null:
                    return MathNode(Subtypes.NULL, new_value)
                elif type(new_value) == MathNode:
                    return new_value
                elif type(new_value) == Constant:
                    return MathNode(Subtypes.CONSTANT, new_value)
                else:
                    return MathNode(Subtypes.NUMBER, new_value)
            else:
                return MathNode(Subtypes.VAR, self.value)
        else:
            return MathNode(self.subtype, self.value, children=tuple(new_children))

def log10(self:MathNode):
    return MathNode(Subtypes.OPERATION, Operations.LOG10, children=as_Node(self))

def ln(self:MathNode):
    return MathNode(Subtypes.OPERATION, Operations.LN, children=as_Node(self))

def sin(self:MathNode):
    return MathNode(Subtypes.OPERATION, Operations.SIN, children=as_Node(self))

def cos(self:MathNode):
    return MathNode(Subtypes.OPERATION, Operations.COS, children=as_Node(self))

def tan(self:MathNode):
    return MathNode(Subtypes.OPERATION, Operations.TAN, children=as_Node(self))

def csc(self:MathNode):
    return MathNode(Subtypes.OPERATION, Operations.CSC, children=as_Node(self))

def sec(self:MathNode):
    return MathNode(Subtypes.OPERATION, Operations.SEC, children=as_Node(self))

def cot(self:MathNode):
    return MathNode(Subtypes.OPERATION, Operations.COT, children=as_Node(self))

def arcsin(self:MathNode):
    return MathNode(Subtypes.OPERATION, Operations.ARCSIN, children=as_Node(self))

def arccos(self:MathNode):
    return MathNode(Subtypes.OPERATION, Operations.ARCCOS, children=as_Node(self))

def arctan(self:MathNode):
    return MathNode(Subtypes.OPERATION, Operations.ARCTAN, children=as_Node(self))

def arccsc(self:MathNode):
    return MathNode(Subtypes.OPERATION, Operations.ARCCSC, children=as_Node(self))

def arcsec(self:MathNode):
    return MathNode(Subtypes.OPERATION, Operations.ARCSEC, children=as_Node(self))

def arccot(self:MathNode):
    return MathNode(Subtypes.OPERATION, Operations.ARCTAN, children=as_Node(self))
    
def floor(self):
    return MathNode(Subtypes.OPERATION, Operations.FLOOR, children=as_Node(self))

def ceil(self):
    return MathNode(Subtypes.OPERATION, Operations.CEIL, children=as_Node(self))


def as_Node(*members):

    node_members = []
    for member in members:
        if type(member) == MathNode:
            node_members.append(member)
        else:
            if type(member) == Null:
                subtype = Subtypes.NULL
            elif type(member) == str:
                subtype = Subtypes.VAR
            elif type(member) == Constant:
                subtype = Subtypes.CONSTANT
            else:
                subtype = Subtypes.NUMBER
            node_members.append(MathNode(subtype, member))

    return tuple(node_members)



def run():

    a = MathNode(Subtypes.NUMBER, 2)
    b = MathNode(Subtypes.NUMBER, 5)
    d = MathNode(Subtypes.VAR, "x")
    c = a+2+(d*"a"*b)#/a//b%a
    c1 = c.simplify_multiple_additions()
    c2 = c.simplify_multiple_multiplications()

    c3 = c1.simplify_multiple_multiplications()
    c4 = c2.simplify_multiple_additions()



    #print(c)
    #print(c1, "add")
    #print(c2, "mul")
    #print()
    print(c3, "add - mul")
    #print(c4, "mul - add")


if __name__ == "__main__":
    run()