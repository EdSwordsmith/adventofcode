from typing import List


def read_decks(input_file: str) -> List[List[int]]:
    with open(input_file) as f:
        decks = f.read().split("\n\n")
    for i in range(2):
        decks[i] = [int(card) for card in decks[i].splitlines()[1:]]
    return decks


def play_round(decks: List[List[int]]):
    cards = []
    for deck in decks:
        cards.append(deck.pop(0))
    
    if cards[0] > cards[1]:
        decks[0].append(cards[0])
        decks[0].append(cards[1])
    else:
        decks[1].append(cards[1])
        decks[1].append(cards[0])


def game_finished(decks: List[List[int]]):
    return len(decks[0]) == 0 or len(decks[1]) == 0


decks = read_decks("input.txt")
while not game_finished(decks):
    play_round(decks)

winner = None
if len(decks[0]) == 0:
    winner = decks[1]
else:
    winner = decks[0]

points = 0
cards = len(winner)
for i, card in enumerate(winner):
    points += card * (cards - i)
print(points)
