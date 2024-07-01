#include <iostream>

// A quantidade de pontos que o jogador ganha depende do caminho que a bolinha seguir
// O jogador pode controlar o percurso da bolinha mudando a posição de algumas portinhas do labirinto.
// A portinha PP está na posição 1 e a portinha RR, na posição 0. Desse jeito, a bolinha vai cair pelo caminho B.

int main() {
    std::string input;
    getline(std::cin, input);

    u_char p_char = input[0];
    u_char r_char = input[2];

    std::string result;

    if (p_char == '0') {
        result = "C\n";
    } else {
        if (r_char == '0') {
            result = "B\n";
        } else {
            result = "A\n";
        }
    }
    std::cout << result;
    return 0;
}
