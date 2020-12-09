#include <stdio.h>

#define PREAMBLE 25

int main(int argc, char const *argv[])
{
    FILE *file = fopen("input.txt", "r");

    int previous[PREAMBLE] = {0};
    int input;
    int i = 0;

    while (fscanf(file, "%d", &input) != EOF)
    {
        if (i >= PREAMBLE)
        {
            int valid = 0;
            for (int j = 0; j < PREAMBLE; j++)
            {
                for (int k = 0; k < PREAMBLE; k++)
                {
                    if (previous[j] + previous[k] == input)
                        valid = 1;
                }
            }

            if (!valid)
            {
                printf("%d\n", input);
                break;
            }
        }
        
        previous[i % PREAMBLE] = input;
        i++;
    }
    
    fclose(file);
    
    return 0;
}
