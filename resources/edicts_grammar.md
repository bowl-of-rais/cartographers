# Regular Grammar for Edicts

EDICT -> Earn N reputation star(s) for each THING
            | Choose a THING and earn N reputation star(s) for each THING

note: *second THING can refer to first THING*

THING -> PIECE | PIECE CONDITION | QUALIFIER PIECE | QUALIFIER PIECE CONDITION

QUALIFIER -> largest | second largest | longest

PIECE -> T space(s)
            | NUM spaces
            | filled space(s)
            | CLUSTER
            | S_PIECE
            | space along an edge
            | PIECE or PIECE
            | NUM different terrain types

CLUSTER -> (NUM) T spaces

S_PIECE -> complete S_PIECE
            | unbroken S_PIECE
            | row
            | column
            | diagonal
            | square
            | NxN square
            | odd-numbered S_PIECE
            | even-numbered S_PIECE
            | S_PIECE and S_PIECE
            | S_PIECE or S_PIECE
            | the edge of the map

CONDITION -> adjacent to PIECE
                | on a PIECE
                | in a PIECE
                | connected to PIECE by PIECE
                | surrounded by PIECE
                | with NUM PIECE
                | containing a PIECE
                | not CONDITION
                | touching the left and bottom edges of the map

NUM -> N | at least N | exactly N | N or more
N -> 0 | 1 | 2 | 3 | ...
T -> [Terrain options]
