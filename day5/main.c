#include <stdio.h>
#include <math.h>

#define ROWS 128
#define COLS 8
#define SEATS 1024 // this is probably wrong, I don't care tho

int seats[SEATS] = {0};

typedef struct
{
    int row;
    int col;
} seat_t;

seat_t get_seat(const char *code) 
{
    seat_t seat;

    int min = 0;
    int max = ROWS - 1;
    for (int i = 0; i < 7; i++)
    {
        int middle = floor((max + min) / 2.0);
        if (code[i] == 'F')
        {
            max = middle;
            seat.row = min;
        }
        else
        {
            min = middle + 1;
            seat.row = max;
        }
    }
    
    min = 0;
    max = COLS - 1;
    for (int i = 7; i < 10; i++)
    {
        int middle = floor((max + min) / 2.0);
        if (code[i] == 'L')
        {
            max = middle;
            seat.col = min;
        }
        else
        {
            min = middle + 1;
            seat.col = max;
        }
    }

    return seat;
}

int get_seat_id(seat_t seat) 
{
    return seat.row * 8 + seat.col;
}


int main(int argc, char const *argv[])
{
    if (argc < 2) return -1;
    FILE *file = fopen(argv[1], "r");
    if (!file) return -1;

    int highest = 0;
    char code[11];

    while (fscanf(file, "%s", code) > 0)
    {
        seat_t seat = get_seat(code);
        int id = get_seat_id(seat);
        seats[id] = 1;
        if (id > highest) 
            highest = id;
    }

    printf("Highest id: %d\n", highest);

    for (int i = 1; i < SEATS - 1; i++)
    {
        if (seats[i - 1] && seats[i + 1] && !seats[i]) 
        {
            printf("My seat id: %d\n", i);
            break;
        }
    }

    fclose(file);

    return 0;
}
