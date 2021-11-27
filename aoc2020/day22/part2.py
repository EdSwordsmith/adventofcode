from typing import List


def read_decks(input_file: str) -> List[List[int]]:
    with open(input_file) as f:
        decks = f.read().split("\n\n")
    for i in range(2):
        decks[i] = [int(card) for card in decks[i].splitlines()[1:]]
    return decks


def should_play_subgame(cards: List[int], decks: List[List[int]]) -> bool:
    return len(decks[0]) >= cards[0] and len(decks[1]) >= cards[1]


def game_finished(decks: List[List[int]]):
    return len(decks[0]) == 0 or len(decks[1]) == 0


def play_round(decks: List[List[int]]):
    if game_finished(decks):
        if len(decks[0]) == 0:
            return 1
        return 0

    # Each player picks their card
    cards = []
    for deck in decks:
        cards.append(deck.pop(0))
    
    winner = 1
    if cards[0] > cards[1]:
        winner = 0
    
    if should_play_subgame(cards, decks):
        copy = [decks[0][:cards[0]], decks[1][:cards[1]]]
        winner = play_game(copy)

    if winner == 0:
        decks[0].append(cards[0])
        decks[0].append(cards[1])
    else:
        decks[1].append(cards[1])
        decks[1].append(cards[0])
    
    return None


def play_game(decks: List[List[int]]):
    store = []
    while not game_finished(decks):
        if str(decks[0]) + str(decks[1]) in store:
            return 0
        store.append(str(decks[0]) + str(decks[1]))
        winner = play_round(decks)
        if winner is not None:
            return winner
    if len(decks[0]) == 0:
        return 1
    return 0


decks = read_decks("input.txt")

if play_game(decks) == 0:
    winner = decks[0]
else:
    winner = decks[1]

points = 0
cards = len(winner)
for i, card in enumerate(winner):
    points += card * (cards - i)
print(points)
