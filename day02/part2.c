#include <stdio.h>
#include <string.h>

int main()
{
    FILE *input = fopen("input.txt", "r");

    int total = 0;
    int min = 0;
    int max = 0;
    char c = 0;
    char password[60];

    while (fscanf(input, "%d-%d %c: %s", &min, &max, &c, password) >= 4)
    {
        int len = strlen(password);
        if (max > len)
            continue;

        if ((password[min - 1] == c && password[max - 1] != c) ||
            password[min - 1] != c && password[max - 1] == c)
        {
            total++;
        }
    }

    printf("%d", total);

    return 0;
}
