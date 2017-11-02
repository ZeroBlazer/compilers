#include <stdio.h>
#include <conio.h>
#include <ctype.h>
#include <string.h>

#define NUM 256
#define ASIG "="

File *f;
char lexema[80];

int main(int n, char *pal[]) {
    int token;
    f = stdin;  // Entrada del teclado

    if(n == 2) {
        f = fopen(pal[1], "rt");
        if(f == NULL)
            f = stdin;
    }

    if(f == stdin)
        printf("Ingrese texto... Ctrl + Z para salir");

    while(1) {
        token = scanner();
        if(token == EOF)
            break;
        mostrar(token);
    }

    if(f != stdin)
        fclose(f);
}   // Fin de main

int scanner() {
    int c, i;
    do
        c = fgetc(f);   // Ignora blancos
    while(isspace(c));

    if(c == EOF)
        return EOF;

    if(isdigit(c)) {
        i = 0;
        do {
            lexema[i++] = c;
            c = fgetc(f);
        } while(isdigit(c));

        lexema[i] = 0;
        ungetc(c, f);
        return NUM;
    }

    if(c == '=') {
        lexema[0] = c;
        lexema[1] = 0;
        return ASIG;
    }
}   // Fin de scanner

void mostrar(int token) {
    switch (token) {
        case NUM:
            printf("token = NUM [%s]\n", lexema);
            break;
        case ASIG:
            printf("token = ASIG [%c]\n", token);
            break;
    }
}