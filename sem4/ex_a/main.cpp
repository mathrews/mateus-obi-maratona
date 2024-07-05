//  mas Vovô João tem um acordo com o banco que garante que ele somente é cobrado se o saldo for
//  menor do que um valor pré-estabelecido.
//
// Dada a movimentação diária da conta do banco do Vovô João, você deve escrever um programa
// que calcule o menor saldo da conta, no período dado.
//
// Entrada:
// A primeira linha da entrada contém dois números inteiros NN e SS que indicam respectivamente
// o número de dias do período de interesse e o saldo da conta no início do período.
//
// Cada uma das NN linhas seguintes contém um número inteiro indicando a movimentação de
// um dia (valor positivo no caso de depósito, valor negativo no caso de retirada).
//
// A movimentação é dada para um período de NN dias consecutivos: a primeira das NN linhas
// corresponde ao primeiro dia do período de interesse, a segunda linha corresponde ao segundo dia, e assim por diante.
//
// Seu programa deve imprimir uma única linha, contendo um único número inteiro, o menor valor de saldo da conta no período dado.

#include <iostream>
#include <sstream>
#include <vector>

using namespace std;

int main() {

    string line1;
    getline(cin, line1);
    stringstream ss(line1);

    string token;
    vector<string> DataNumbers;
    char delimiter = ' ';

    while (getline(ss, token, delimiter)) {
        DataNumbers.push_back(token);
    }

    vector<int> Data;

    transform(DataNumbers.begin(), DataNumbers.end(), back_inserter(Data), [](const string& str) {return std::stoi(str);});

    vector<int> minorVals;

    if (Data[0] == 0) {
        cout << Data[1] << endl;
    } else {
        for (int i = 0; i < Data[0]; i++) {
            int input;
            cin >> input;
            if (input > 0) {
                Data[1] += input;
                continue;
            } else {
                Data[1] = Data[1] - (input * -1);
                minorVals.push_back(Data[1]);
            }
        }
        sort(begin(minorVals), end(minorVals));
    }
    cout << minorVals[0];

    return 0;
}
