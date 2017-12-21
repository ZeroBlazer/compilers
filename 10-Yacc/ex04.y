%{
#include <stdio.h>
#include <ctype.h>
#include <math.h>
%}

%token num
%left '+'
%left '-'

%%
    exp: exp '+' term { printf("+ "); } ;
    exp: exp '-' term { printf("- "); } ;
    exp: term ;
    term: '(' exp ')';
    term: num { printf("%d ", $1); } ;
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