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
        int num = 0;
        int len = strlen(password);
        for (int i = 0; i < len; i++)
        {
            if (password[i] == c)
                num++;
        }

        if (num >= min && num <= max)
        {
            total++;
        }
    }

    printf("%d", total);

    return 0;
}
