#include <stdio.h>
#include <stdlib.h>

typedef struct
{
    int ns; // North - South
    int ew; // East - West
} pos_t;

pos_t waypoint;
pos_t ship;

void handle_command(const char command, const int arg)
{
    switch (command)
    {
    case 'N':
        waypoint.ns += arg;
        break;
    case 'S':
        waypoint.ns -= arg;
        break;
    case 'E':
        waypoint.ew += arg;
        break;
    case 'W':
        waypoint.ew -= arg;
        break;
    case 'R':
        for (int i = 0; i < arg; i += 90)
        {
            int ns = waypoint.ns;
            waypoint.ns = -waypoint.ew;
            waypoint.ew = ns;
        }
        break;
    case 'L':
        for (int i = 0; i < arg; i += 90)
        {
            int ns = waypoint.ns;
            waypoint.ns = waypoint.ew;
            waypoint.ew = -ns;
        }
        break;
    case 'F':
        ship.ns += waypoint.ns * arg;
        ship.ew += waypoint.ew * arg;
        break;
    }
}

int main(int argc, char const *argv[])
{
    ship.ns = 0;
    ship.ew = 0;
    waypoint.ns = 1;
    waypoint.ew = 10;

    FILE *file = fopen("input.txt", "r");
    char command;
    int arg;

    while (fscanf(file, "%c%d", &command, &arg) != EOF)
    {
        handle_command(command, arg);
    }

    printf("%d %d -> %d\n", ship.ew, ship.ns, abs(ship.ns) + abs(ship.ew));

    fclose(file);

    return 0;
}
