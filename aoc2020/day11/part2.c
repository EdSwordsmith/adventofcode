#include <stdio.h>
#include <string.h>

#define MIN(X, Y) (((X) < (Y)) ? (X) : (Y))
#define MAX(X, Y) (((X) > (Y)) ? (X) : (Y))

#define COLS 92
#define ROWS 99

typedef enum { FLOOR, EMPTY_SEAT, OCCUPIED_SEAT } pos_t;

pos_t positions[ROWS * COLS];

int get_index(int row, int col)
{
    return row * COLS + col;
}

void fill_positions(FILE *file)
{
    char input[COLS + 1];
    int row = 0;

    while (fscanf(file, "%s", &input) != EOF)
    {
        for (int i = 0; i < COLS; i++)
        {
            int index = get_index(row, i);
            if (input[i] == 'L')
                positions[index] = EMPTY_SEAT;
            else
                positions[index] = FLOOR;
        }

        row++;
    }
}

int check_direction(int row, int col, int dir_r, int dir_c)
{
    int pos_r = row + dir_r;
    int pos_c = col + dir_c;

    while (pos_r >= 0 && pos_r < ROWS && pos_c >= 0 && pos_c < COLS)
    {
        int index = get_index(pos_r, pos_c);
        if (positions[index] == EMPTY_SEAT)
            return 0;
        else if (positions[index] == OCCUPIED_SEAT)
            return 1;
        
        pos_r += dir_r;
        pos_c += dir_c;
    }
    
    return 0;
}

int get_adjacent_occupied(int row, int col)
{
    int count = 0;

    for (int i = -1; i <= 1; i++)
    {
        for (int j = -1; j <= 1; j++)
        {
            if (i == 0 & j == 0) continue;
            count += check_direction(row, col, i, j);
        }
    }
    
    return count;
}

pos_t position_tick(int row, int col)
{
    int index = get_index(row, col);
    int adjacent = get_adjacent_occupied(row, col);
    pos_t state = positions[index];

    if (state == EMPTY_SEAT && adjacent == 0)
        state = OCCUPIED_SEAT;
    else if (state == OCCUPIED_SEAT && adjacent >= 5)
        state = EMPTY_SEAT;

    return state;
}

int positions_tick()
{
    int changed = 0;
    pos_t new_state[ROWS * COLS];

    for (int i = 0; i < ROWS; i++)
    {
        for (int j = 0; j < COLS; j++)
        {
            pos_t state = position_tick(i, j);
            int index = get_index(i, j);
            new_state[index] = state;
            if (positions[index] != state)
                changed = 1;
        }
    }

    // Copy new state into the positions
    memcpy(positions, new_state, sizeof(positions));
    
    return changed;
}

int count_occupied()
{
    int count = 0;

    for (int i = 0; i < ROWS * COLS; i++)
    {
        if (positions[i] == OCCUPIED_SEAT)
            count++;
    }

    return count;
}

int main(int argc, char const *argv[])
{
    FILE *file = fopen("input.txt", "r");
    fill_positions(file);
    fclose(file);

    while (positions_tick()); 

    printf("%d\n", count_occupied());

    return 0;
}
