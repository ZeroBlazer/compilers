%{
#include <stdio.h>
#include <ctype.h>
#include <math.h>
%}

%token OR AND TRUE FALSE NOT PARI PARD
%left ENTER

%%
    EV: E           { if ($1 == 1) printf("valor = true\n");
                      else printf("valor = false\n"); } ;
    E:  T           { $$ = $1; } ;
    E:  T OR E      { $$ = $1 | $3; } ;
    T:  F           { $$ = $1; };
    T:  F AND T     { $$ = $1 | $3; } ; 
    F:  TRUE        { $$ = 1; } ;
    F:  FALSE       { $$ = 1; } ;
    F:  NOT F       { if ($2 == 1) $$ = 0;
                      else $$ = 1; } ;
    F: '(' E ')'    { $$ = $2; } ;
%%

yylex() {
    int c;
    char lexema[30];
    char *p;
    do c = getchar(); while (c == ' ' || c == '\t');
    if (c == '\n') return EOF;
    
    if (isalpha(c)) {
        p = lexema;
        do { *p++ = c; c = getchar(); } while (isalpha(c));
        ungetc(c, stdin);
        *p = 0;
        if (strcmp(lexema, "or") == 0) return OR;
        if (strcmp(lexema, "and") == 0) return AND;
        if (strcmp(lexema, "true") == 0) return TRUE;
        if (strcmp(lexema, "false") == 0) return FALSE;
        if (strcmp(lexema, "not") == 0) return NOT;
        yyerror("Identificador ilegal");
    }

    if (c == '(' || c == ')') return c;
    
    yyerror("Caracter ilegal");
}

yyerror(char *m) {
    printf("\n%sn", m);
    // getch();
    exit(0);
}

main() {
    yyparse();
    // getch();
}