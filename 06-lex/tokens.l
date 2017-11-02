%%
[0123456789]+           printf("Token = NUM\n");
[a-zA-Z][a-zA-Z0-9]*    printf("Token = ID\n");
" +	"					printf("Token = MAS\n");
%%