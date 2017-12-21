%{
#include <stdio.h>
#include <ctype.h>
#include <math.h>
%}

%token ID

%%
    S: 'x' S;
    S: A B 'c';
    A: 'a';
    B: 'b';
%%

yylex() {
    int c;

    do c = getchar(); while (c == ' ' || c == '\t');

    if ( c == '\n') return EOF;

    if (isalpha(c))
        return c;
    
    yyerror("caracter ilegal");
}

yyerror(char *m) {
    printf("%s", m);
    exit(0);
}

main() {
    while (1) {
        yyparse();
    }
}