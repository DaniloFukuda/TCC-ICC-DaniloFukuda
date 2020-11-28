#include <stdio.h>
typedef struct{
    int numero;
    float saldo;
}Bconta;
void imprimir(Bconta *conta, int i){
int j = 0;
for(j = 0; j<=i; j++){
    printf("conta: %d\n",conta[j].numero);
    printf("saldo da conta:%f\n",conta[j].saldo);
}
return NULL;
}
int main(void){
int num, x = 0, i = 0, j = 0,k = 0,l = 0,apagar = 0, acesso = 0;
float dep = 0, saque = 0;
Bconta *conta;
conta = (Bconta *) malloc(20 * sizeof(Bconta));
conta[i].numero = i;
conta[i].saldo = 0;
while(x == 0){
    printf("Escolha qual menu quer acessar\n");
    printf("1 = Menu de administrador\n");
    printf("2 = Menu de usuario\n");
    printf("3 = Sair\n");
    scanf("%d",&num);
        switch(num){
            case 1:
                printf("1 - Mostrar contas\n");
                printf("2 - Criar contas\n");
                printf("3 - Apagar contas\n");
                printf("4 - Sair\n");
                scanf("%d",&num);
                switch(num){
                    case 1:
                        imprimir(&conta[0],i);
                        break;
                    case 2:
                        printf("Quantas contas serao criadas?\n");
                        scanf("%d",&k);
                        if(i + k<20){
                            for (j = 0; j <= k; ++j){
                                conta[i].numero = i;
                                conta[i].saldo = 0;
                                printf("saldo da conta %d:%f\n",conta[j].numero,conta[j].saldo);
                                i++;
                            }
                            i--;
                            printf("Valor de i: %d\n",i);
                        }
                        else
                            printf("Valor invalido!\n");
                        break;
                    case 3:
                        printf("Quantas contas vao ser apagadas?");
                        scanf("%d",&apagar);
                        if(i > apagar){
                            i = i - apagar;
                        }
                        else
                            printf("Numero invalido\n");
                        break;
                    case 4:
                        printf ("Saiu com sucesso!\n");
                        x= 1;
                        break;
                    default :
                        printf ("Valor invalido!\n");
                }
        case 2:
                printf("Qual conta sera acessada?");
                scanf("%d",&acesso);
                if(acesso <= i){
                    printf("Selecione a operacao:\n");
                    printf("1 = Deposito\n");
                    printf("2 = Saque\n");
                    printf("3 - Saldo\n");
                    printf("4 = Sair\n");
                    scanf("%d",&num);
                    switch(num){
                        case 1:
                            printf("Qual a quantia a ser depositada?\n");
                            scanf("%f",&dep);
                            conta[acesso].saldo = conta[acesso].saldo + dep;
                            printf("O saldo e de: %f\n",conta[acesso].saldo);
                        break;
                        case 2:
                            printf("Qual a quantia a ser sacada?\n");
                            scanf("%f",&saque);
                            if(saque <= conta[acesso].saldo){
                                conta[acesso].saldo = conta[acesso].saldo - saque;
                                printf("O saldo e de: %f\n",conta[acesso].saldo);
                            }
                            else{
                                printf("Valor invalido, saque > saldo\n");
                            }
                            break;
                        case 3:
                            printf("O saldo da conta %d e de: %f\n",conta[acesso].numero,conta[acesso].saldo);
                            break;
                        case 4:
                            printf ("Saiu com sucesso!\n");
                            x = 1;
                            break;
                        default :
                            printf ("Valor invalido!\n");
                    }
                }
                else
                    printf("Acesso invalido!\n");
                break;
        case 3:
                printf ("Saiu com sucesso!\n");
                x = 1;
                break;
        default :
                printf ("Valor invalido!\n");
    }
}
free(conta);
return 0;
}
