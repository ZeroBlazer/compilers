%{
    #include<stdio.h>
%}
%%
[ \t\n]   ;
[0123456789]+                       printf("Token = Int[%s]\n", yytext);
[-]?[0123456789]*"."[0123456789]+       printf("Token = Real[%s]\n", yytext);
"while"                             printf("Token = WHILE[%s]\n", yytext);
"if"                                printf("Token = IF[%s]\n", yytext);
[a-zA-Z][a-zA-Z0-9]*                printf("Token = ID[%s]\n", yytext);
[+]                                 printf("Token = Mas[%s]\n", yytext);
[-]                                 printf("Token = Menos[%s]\n", yytext);
[*]                                 printf("Token = Multiplicacion[%s]\n", yytext);
[/]                                 printf("Token = Division[%s]\n", yytext);
[;]                                 printf("Token = PuntoYComa[%s]\n", yytext);
"<"                                 printf("Token = Menor[%s]\n", yytext);
"<="                                printf("Token = MenorIgual[%s]\n", yytext);
">"                                 printf("Token = Mayor[%s]\n", yytext);
">="                                printf("Token = MayorIgual[%s]\n", yytext);
"="                                 printf("Token = Asignacion[%s]\n", yytext);
"=="                                printf("Token = Igualdad[%s]\n", yytext);
"!"                                 printf("Token = Negacion[%s]\n", yytext);
"!="                                printf("Token = Diferencia[%s]\n", yytext);
"\""                                printf("Token = ComillaDoble[%s]\n", yytext);
"("                                 printf("Token = LeftParentheses[%s]\n", yytext);
")"                                 printf("Token = RightParentheses[%s]\n", yytext);
"["                                 printf("Token = LeftBracket[%s]\n", yytext);
"]"                                 printf("Token = RightBracket[%s]\n", yytext);
"{"                                 printf("Token = LeftBrace[%s]\n", yytext);
"}"                                 printf("Token = RightBrace[%s]\n", yytext);
"//"(.*[ \t]*.*)*[\n]+              printf("Token = Line Comment[//]\n", yytext);
"/*"(.*[ \t]*.*[\n]*)*"*/"          printf("Token = Block Comment[/* */]\n", yytext);
%%
int main(int ac, char **av) {
    FILE    *fd;

    if (ac == 2) {
        if (!(fd = fopen(av[1], "r"))) {
            perror("Error: ");
            return (-1);
        }

        yyset_in(fd);
        yylex();
        fclose(fd);
    }
    else
        printf("Usage: ./scanner <filename>\n");

    return (0);
}