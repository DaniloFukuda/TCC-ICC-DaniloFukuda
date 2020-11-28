#include <stdio.h>
main(void){
int n = 1;
float saldo = 1000;

printf("O numero total de contas e:%d\n",n);
saldo = saldo + 1000;
printf("Saldo: %f\n",saldo);
saldo = saldo - 1000;
printf("Saldo: %f\n",saldo);
saldo = saldo * 2;
printf("Saldo: %f\n",saldo);
saldo = saldo/2;
printf("Saldo: %f\n",saldo);
return 0;
}
