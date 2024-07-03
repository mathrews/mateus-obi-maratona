#include <iostream>
#include <sstream>
#include <vector>

using namespace std;

int main()
{
    for (int i = 0; i < 30; i++) {
        string input1;
        std::string linha1;
        getline(cin, input1);
        if (input1[0] == '0') {
            break;
        } else {
            getline(cin, linha1);

            stringstream ss(linha1);

            string token;
            vector<string> DataNumbers;
            char delimiter = ' ';

            while (getline(ss, token, delimiter)) {
                DataNumbers.push_back(token);
            }
            std::vector<int> Data;

            transform(DataNumbers.begin(), DataNumbers.end(), back_inserter(Data), [](const string& str) {return std::stoi(str);});

            int time = 0;
            if (input1 == "1") {
                time += 10;
                cout << time << "\n";
            } else {
                for (int i = 0; i < Data.size(); i++) {
                    if (i == 0) {
                        time += 10;
                        continue;
                    } else {
                        if (Data[i] - Data[i - 1] >= 10) {
                            time += 10;
                        } else {
                            time += Data[i] - Data[i - 1];
                        }
                    }
                }
                cout << time << "\n";
            }
        }
    }
    return 0;
}
