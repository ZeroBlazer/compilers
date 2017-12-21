#include <bits/stdc++.h>
// # include <stdio.h> 
// # include <math.h> 
// // # include <conio.h>
// # include <ctype.h> 
// # include <string>

#define FLOAT 256 
#define MAX 257 
#define MAS '+' 
#define MENOS '-'
#define MUL '*'
#define DIV '/' 
#define PARI '('
#define PARD ')'
#define FIN '-1'
#define COMA ','

using namespace std;

int ERROR; 
int Leertoken; 
float val; //val, es el valor del token FLOAT 
char lexema [80]; 
float E(), T(), F(); 
int scanner();

int main() {
    float result; 
    ERROR = 0; 
    Leertoken = 1; 
    // clrscr();
    result = E(); 
    if (ERROR) 
        printf("Error de sintaxis\n"); 
    else 
        printf("valor = %g\n", result); 
    // getch(); 
}

float E() {
    float v1, v2; 
    int tok;
    v1 = T();
    tok = scanner(); 
    while (tok == MAS || tok == MENOS) {
        v2 = T();
        if (tok == MAS)
            v1 = v1 + v2;
        else
            v1 = v1 - v2;
        tok = scanner(); 
    }
    Leertoken = 0; 
    return v1;
}

float T() {
    float f1, f2;
    int tok;
    f1 = F();
    tok = scanner();

    while (tok == MUL || tok == DIV) {
        f2 = F();
        if (tok == MUL)
            f1 = f1 * f2;
        else
            f1 = f1 / f2;
        tok = scanner();
    }

    Leertoken = 0;
    return f1;
}

float F() {
    float V, V1;
    int tok;
    tok = scanner();

    if (tok == FLOAT)
        return val;

    if (tok == PARI) {
        V = E();
        tok = scanner();

        if (tok != PARD) {
            ERROR++;
            return 0;
        }

        return V;
    }

    if (tok == MAX) {
        tok = scanner();
        
        if (tok != PARI) {
            ERROR++;
            return 0;
        }

        V = E();
        tok = scanner();

        if (tok != COMA) {
            ERROR++;
            return 0;
        }
        V1 = E();
        tok = scanner();

        if (tok != PARD) {
            ERROR++;
            return 0;
        }

        return (V > V1) ? V : V1;
    }

    Leertoken = 0;
}

int scanner() { 
    static int token; 
    int c, i; 
    if (Leertoken == 0) {
        Leertoken = 1;
        return token;
    }

    do c = getchar();
    while (c == ' ' || c == '\t');

    if (c == '\n')
        return token = FIN;

    if (isdigit(c)) { 
        ungetc(c, stdin); 
        scanf("%f", &val); 
        return token = FLOAT;
    }

    if (c == MAS || c == MENOS)
        return token = c; 
    
    if (c == MUL || c == DIV)
        return token = c;
    
    if (c == PARI || c == PARD)
        return token = c; 

    if (c == COMA)
        return token = c;

    if (isalpha(c)) {
        i = 0; 
        do {
            lexema[i++] = c;
            c = getchar();
        } while (isalpha(c)); 
        lexema[i] = 0; 
        ungetc(c, stdin); 
        if (strcmp(lexema, "MAX") == 0) 
            return token = MAX; 
    }
}