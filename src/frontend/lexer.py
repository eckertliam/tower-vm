import re
from enum import Enum, auto

# loc for storing textual position
class Loc:
    def __init__(self, line: int, col: int) -> None:
        self.line = line
        self.col = col
        
    def __repr__(self) -> str:
        return f"Loc: {self.line}:{self.col}"


def index_to_loc(src: str, idx: int) -> Loc:
    line = 1
    col = 1
    for i in range(0, idx):
        if src[i] == '\n':
            line += 1
            col = 1
        else:
            col += 1
    return Loc(line, col)


# span of elements in source code
class Span:
    def __init__(self, start: int, end: int) -> None:
        self.start = start
        self.end = end
        
    def __repr__(self) -> str:
        return f"Span: {self.start} - {self.end}"
    
    def as_loc(self, src: str) -> Loc:
        return index_to_loc(src, self.start)
    
    def as_str(self, src: str) -> str:
        return src[self.start:self.end]
    
class TokenType(Enum):
    Integer = auto()
    Float = auto()
    String = auto()
    Boolean = auto()
    Identifier = auto()
    Keyword = auto()
    Punctuation = auto()
    Operator = auto()
    EOF = auto()
    

class Token:
    def __init__(self, lit: str | bool | int | float, start: int, end: int, _type: TokenType) -> None:
        self.lit = lit
        self.start = start
        self.end = end
        self._type = _type
        
    def __repr__(self) -> str:
        return f"Token: {self.lit}\n Type: {self._type}\n  Span: {self.start} - {self.end}"
    
    def loc(self, src: str) -> Loc:
        return index_to_loc(src, self.start)

NUM_PAT = re.compile(r'^-?\d+(?:\.\d+)?$')
STRING_PAT = re.compile(r'([^"\\]*(\\.[^"\\]*)*)')
IDENT_PAT = re.compile(r'[a-zA-Z_][a-zA-Z0-9_]*')

KEYWORDS = [
    'if', 'else', 'match', 'while', 'for', 'in', 'return', 'break',
    'let', 'type', 'fn', 'require', 'provide'
]

PUNCT = [
    '(', ')', '{', '}', '[', ']', 
    ';', ',', '.', ':', '?', '|',
    '::', '->', '=>', '<<', '>>',
    '!',
]

OPS = [
    '+', '-', '*', '/', '%', '=', '^',
    '==', '!=', '>', '<', '>=', '<=',
    '&&', '||',
]

    
class Lexer:
    def __init__(self) -> None:
        self.tokens = []
        self.idx = 0
        self.src = ""
        
    def peek(self) -> str:
        return self.src[self.idx + 1]
    
    # first number in return is either 0 for int or 1 for float
    # second number is length of number
    # third number is the number itself
    def lex_num(self) -> list[int | float]:
        m = NUM_PAT.match(self.src[self.idx:])
        if m is None:
             raise Exception(f"Failed to lex float at {index_to_loc(self.src, self.idx)}")
        n = m.group(0)
        length = len(n)
        try:
            return [0, length, int(n)]
        except ValueError:
            return [1, length, float(n)]
    
    # returns string without quotes
    def lex_string(self) -> str:
        m = STRING_PAT.match(self.src[self.idx:])
        if m is None:
            raise Exception(f"Failed to lex string at {index_to_loc(self.src, self.idx)}")
        return m.group(1)
    
    # returns identifier
    def lex_ident(self) -> str:
        m = IDENT_PAT.match(self.src[self.idx:])
        if m is None:
            raise Exception(f"Failed to lex identifier at {index_to_loc(self.src, self.idx)}")
        ident = m.group(0)
        return ident
    
    # returns list of tokens
    def lex(self, src: str) -> list[Token]:
        self.src = src
        srclen = len(src)
        while self.idx < srclen:
            ch = self.src[self.idx]
            if ch.isdigit():
                flag, length, num = self.lex_num()
                start = self.idx
                self.idx += length
                end = self.idx
                if flag == 0:
                    self.tokens.append(Token(str(num), start, end, TokenType.Integer))
                else:
                    self.tokens.append(Token(num, start, end, TokenType.Float))
            elif ch == '"':
                string = self.lex_string()
                length = len(string) + 2
                start = self.idx
                self.idx += length
                end = self.idx
                self.tokens.append(Token(string, start, end, TokenType.String))
            elif ch.isalpha():
                ident = self.lex_ident()
                length = len(ident)
                start = self.idx
                self.idx += length
                end = self.idx
                if ident in KEYWORDS:
                    self.tokens.append(Token(ident, start, end, TokenType.Keyword))
                else:
                    self.tokens.append(Token(ident, start, end, TokenType.Identifier))
            elif ch in OPS or ch in PUNCT:
                atom = ch
                loc_idx = self.idx + 1
                while self.src[loc_idx] not in [' ', '\n', '\t']:
                    atom += self.src[loc_idx]
                    loc_idx += 1
                length = len(atom)
                start = self.idx
                self.idx = loc_idx
                end = loc_idx
                if atom in OPS:
                    self.tokens.append(Token(atom, start, end, TokenType.Operator))
                elif atom in PUNCT:
                    self.tokens.append(Token(atom, start, end, TokenType.Punctuation))
                else:
                    raise Exception(f"Failed to lex operator at {Span(start, end).as_loc(self.src)}")
            elif ch in [' ', '\n', '\t']:
                self.idx += 1
            else:
                loc = index_to_loc(self.src, self.idx)
                raise Exception(f"Failed to lex token at {loc}")
        self.tokens.append(Token("", self.idx, self.idx, TokenType.EOF))
        return self.tokens

