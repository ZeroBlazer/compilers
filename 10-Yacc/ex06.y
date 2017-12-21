%{
#include <stdio.h>
#include <ctype.h>
#include <math.h>
%}

%token num
%left '+'
%left '-'

%%
    prg :   exp { printf("%d \n", $1); };

    exp :   exp '+' term    { $$ = $1 + $3; }
        |   exp '-' term    { $$ = $1 - $3; }
        |   term            { $$ = $1; };
    
    term:   term '*' factor { $$ = $1 * $3; }
        |   factor          { $$ = $1; }

    factor: '(' exp ')'     { $$ = $2; }
          | num             { $$ = $1; };
%%

yylex() {
    int c;
    
    do c = getchar(); while(c == ' ' || c == '\t');
    
    if ( c == '\n') return EOF;
    
    if (isdigit(c)) {
        ungetc(c, stdin);
        scanf("%d", &yylval);
        return(num);
    }
    
    return c;
}

yyerror(char *m) {
    printf("%s", m);
    exit(0);
}

main() {
    while (1) {
        yyparse();
        printf("\n");
    }
}