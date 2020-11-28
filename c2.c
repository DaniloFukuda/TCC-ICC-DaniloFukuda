#include <stdio.h>
main(void){
int num;
float saldo = 1000, dep = 0, saque = 0;

printf("Selecione a operacao:\n");
printf("1 = Deposito\n");
printf("2 = Saque\n");
scanf("%d",&num);
switch(num){
    case 1:
        printf("Qual a quantia a ser depositada?\n");
        scanf("%f",&dep);
        saldo = saldo + dep;
        printf("O saldo e de: %f",saldo);
        break;
    case 2:
        printf("Qual a quantia a ser sacada?\n");
        scanf("%f",&saque);
        if(saque <= saldo){
            saldo = saldo - saque;
            printf("O saldo e de: %f\n",saldo);
            break;
        }
        else
            printf("Valor invalido, saque > saldo\n");
            break;
    default :
        printf ("Valor invalido!\n");

}
return 0;
}
