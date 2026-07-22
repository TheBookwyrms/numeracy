# numeracy

A math library with linear algebra and other math capabilities. This README will be expanded with detail and information in time.



## To-Do list

- Versioning
    - [ ] set version 2.0.0 when linear algebra and overall math-tree capabilities are generally finished
        - [ ] merge `working_branch` into `reference`, upon version 2.0.0 


- Documentation
    - [ ] make the README nice and complete
        - [ ] discuss the compiler-like nature of MathTree<T> ?
    - [ ] create full pdf document detailing construction, mechanisms, and methods by which numeracy works


- notation
    - [ ] sigma (summation) notation


- geometry
    - triangle stuff
        - [ ] area of triangles


- code
    - [ ] finish all things with a `panic!()` invocation


- Linear Algebra
    - [ ] optimization of linear equations with matrices
    - [ ] least squares solutions to linear equations
    - [ ] remove errors from addition/subtraction for matrices and vectors ????
    - [ ] matrix constructor for triangular matrices
    - [ ] matrix multiplication
        - [ ] add MatrixMultiplication trait to allow matrices to do matrix multiplication to both matrices and vectors, but not the other way around
    - [ ] matrix exponentiation by positive integer values
    - [ ] minor of a matrix at an index
    - [ ] cofactor of a matrix at an index
    - [ ] cofactor matrix
    - [ ] adjoint matrix
    - [ ] inverse of a matrix by [DeterminantAdjointMethod, GaussianElimination, GaussJordanElimination] enum
    - [ ] get echelon form of a matrix
        - [ ] ensure all null rows are at the bottom of the echelon matrix
    - [ ] rank of a matrix
    - [ ] angle between two vectors by dot product
    - [ ] formula: ||aù + bò||^2 expansion ?
    - [ ] projection of one vector onto another
    - [ ] ability to test for linear independance among vectors
    - [ ] change of base for vectors
    - [ ] angle between two vectors by cross product lagrange identity
    - [ ] mixed product of vectors ?
    - [ ] define struct for Line (in general)
        - [ ] by a point and direction
        - [ ] use vector equations
        - [ ] use parametric equations
    - [ ] relative position of two lines on plane (intersecting, parallel and distinct, parallel and confounding)
    - [ ] distance from a point to a line
    - [ ] distance between two parallel lines (or just two lines in general ?)
    - [ ] define struct for Plane (in general)
        - [ ] by a point and two internal (non-parallel) directions
        - [ ] by a point and a normal vector
        - [ ] use vector equations
        - [ ] use parametric equations
    - [ ] relative position of two lines in space (intersecting, parallel and distinct, parallel and confounding, never intersecting)
    - [ ] distance between a line and a point in space
    - [ ] distance between non-intersecting (skew) lines
    - [ ] relative position of two planes (parallel+distinct, parallel+confounding, not parallel (intersecting along a line))
    - [ ] relative position of a line and a plane (parallel and outside plane, parallel inside plane, not parallel (intersecting at a point))


- calculus
    - [ ] differentiability rules for all basic operations
    - [ ] differentiability rules for logarithms/exponents
    - [ ] differentiability rules for trigonometric functions
    - [ ] some simple limits ?
        - [ ] simple l'hopital rule limits ?
        - [ ] more complex limits ?
    - [ ] implicit differentiation
    - [ ] related rates ?
    - [ ] concavity
    - [ ] simple direct integrals
    - [ ] easy trig/algebra substitutions?
    - [ ] complete integration mechanism ?
        - [ ] integration by parts
        - [ ] partial fractions
        - [ ] trig substitutions
        - [ ] integrals of trigonometric functions
    - [ ] definite integrals
    - [ ] improper integrals (if limits are implemented)
        - [ ] convergence and divergence of functions
    - [ ] average value of a function
    - [ ] volumes by rotation about a line
        - [ ] cylindrical shells method
        - [ ] washers/disks method
    - [ ] area between two curves
    - [ ] sequences
        - [ ] arithmetic operations on sequences
        - [ ] infinite sequences
    - [ ] series (sum of a sequence)
        - [ ] convergence/divergence of series
            - [ ] geometric series
            - [ ] telescoping series
            - [ ] test for divergence
            - [ ] integral test
            - [ ] limit comparison test
            - [ ] absolute convergence
            - [ ] ratio test
        - [ ] approximation of error with integrals
        - [ ] power series
            - [ ] interval of convergence
                - [ ] convergence/divergence of endpoints of interval
            - [ ] radius of convergence
            - [ ] taylor series representations of functions
    - [ ] stuff from cal 3 course




NOTE to self: when making a full write-up of the README, talk about the compiler like aspects of using a tree method for MathTree and how it works with differentiating math expressions and how it traverses the tree




# old README

to-do list:
- [ ] fix up matrix rank code
- [ ] move null rows to bottom of an echelon matrix
- [ ] add optimization for matrices
- [ ] least squares solving for matrices
- [ ] finish adding material from linear algebra course
- [ ] matrix exponentiation
- [ ] add MatrixMultiplication trait so matrices can matmul both Matrix and Vector

- [ ] read through code to find relations, and what type restrictions are needed where, then simplify
- [ ] write tests
- [ ] write documentation

- [ ] add variables (and therefore Numbers and Operations and etc)
- [ ] trigonometry
- [ ] add statistics stuff
- [ ] ArbitraryPrecisionNumber + OperationTree
    - [ ] add to Matrix and Vector
    - make it a trait usable alongside actual numbers?
- [ ] add solution set for infinite solutions gauss/gauss-jordan elimination
- [ ] osculation circle and plane
- [ ] definition of a distance metric of power n
- [ ] function stuff (related to R (as in space of variales (R1, R2, R3, ..., Rn)), and definition)
- [ ] calculus stuff
    - [ ] differentiation