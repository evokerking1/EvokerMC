"""
Simple Economy System
Manages a basic economy with currency and transactions
"""
import game
import json

# Configuration
CURRENCY_NAME = "coins"
STARTING_BALANCE = 100

# In-memory player balances (in full implementation, would use persistent storage)
player_balances = {}

def log(message):
    """Helper function for logging"""
    game.py_log(f"[Economy] {message}")

def get_balance(player_id):
    """Get a player's balance"""
    if player_id not in player_balances:
        player_balances[player_id] = STARTING_BALANCE
        log(f"New player {player_id} registered with {STARTING_BALANCE} {CURRENCY_NAME}")
    return player_balances[player_id]

def set_balance(player_id, amount):
    """Set a player's balance"""
    player_balances[player_id] = amount
    log(f"Set {player_id} balance to {amount} {CURRENCY_NAME}")

def add_money(player_id, amount):
    """Add money to a player's account"""
    if amount <= 0:
        log(f"Invalid amount: {amount}")
        return False
    
    current = get_balance(player_id)
    new_balance = current + amount
    set_balance(player_id, new_balance)
    
    # In full implementation:
    # game.send_message(player_id, f"You received {amount} {CURRENCY_NAME}!")
    log(f"Added {amount} {CURRENCY_NAME} to {player_id}")
    return True

def remove_money(player_id, amount):
    """Remove money from a player's account"""
    if amount <= 0:
        log(f"Invalid amount: {amount}")
        return False
    
    current = get_balance(player_id)
    if current < amount:
        log(f"Insufficient funds: {player_id} has {current}, needs {amount}")
        return False
    
    new_balance = current - amount
    set_balance(player_id, new_balance)
    
    # In full implementation:
    # game.send_message(player_id, f"You spent {amount} {CURRENCY_NAME}!")
    log(f"Removed {amount} {CURRENCY_NAME} from {player_id}")
    return True

def transfer_money(from_player, to_player, amount):
    """Transfer money between players"""
    if amount <= 0:
        log(f"Invalid transfer amount: {amount}")
        return False
    
    if remove_money(from_player, amount):
        add_money(to_player, amount)
        log(f"Transfer: {from_player} -> {to_player}: {amount} {CURRENCY_NAME}")
        
        # In full implementation:
        # game.send_message(from_player, f"You sent {amount} {CURRENCY_NAME} to {to_player}")
        # game.send_message(to_player, f"You received {amount} {CURRENCY_NAME} from {from_player}")
        return True
    
    return False

def buy_item(player_id, item_id, price):
    """Handle item purchase"""
    if remove_money(player_id, price):
        log(f"{player_id} purchased {item_id} for {price} {CURRENCY_NAME}")
        # In full implementation:
        # game.give_item(player_id, item_id, 1)
        # game.send_message(player_id, f"You bought {item_id} for {price} {CURRENCY_NAME}!")
        return True
    else:
        log(f"{player_id} cannot afford {item_id} (costs {price} {CURRENCY_NAME})")
        # In full implementation:
        # game.send_message(player_id, f"You need {price} {CURRENCY_NAME} to buy {item_id}")
        return False

def sell_item(player_id, item_id, price):
    """Handle item sale"""
    # In full implementation:
    # game.remove_item(player_id, item_id, 1)
    add_money(player_id, price)
    log(f"{player_id} sold {item_id} for {price} {CURRENCY_NAME}")
    # In full implementation:
    # game.send_message(player_id, f"You sold {item_id} for {price} {CURRENCY_NAME}!")
    return True

def get_leaderboard(top_n=10):
    """Get top players by balance"""
    sorted_players = sorted(player_balances.items(), key=lambda x: x[1], reverse=True)
    return sorted_players[:top_n]

def show_leaderboard():
    """Display economy leaderboard"""
    log("=== Economy Leaderboard ===")
    leaderboard = get_leaderboard()
    for i, (player_id, balance) in enumerate(leaderboard, 1):
        log(f"{i}. {player_id}: {balance} {CURRENCY_NAME}")

# Command handlers
def handle_balance_command(player_id):
    """Handle balance check command"""
    balance = get_balance(player_id)
    log(f"{player_id} checked balance: {balance} {CURRENCY_NAME}")
    # In full implementation:
    # game.send_message(player_id, f"Your balance: {balance} {CURRENCY_NAME}")

def handle_pay_command(from_player, to_player, amount):
    """Handle payment command"""
    if transfer_money(from_player, to_player, amount):
        log(f"Payment successful: {from_player} -> {to_player}: {amount}")
    else:
        log(f"Payment failed: {from_player} -> {to_player}: {amount}")

# Initialize
log("Economy System loaded!")
log(f"Currency: {CURRENCY_NAME}")
log(f"Starting balance: {STARTING_BALANCE}")
log("Available commands:")
log("  - /balance - Check your balance")
log("  - /pay <player> <amount> - Send money to another player")
log("  - /buy <item> - Buy an item")
log("  - /sell <item> - Sell an item")
log("  - /baltop - Show economy leaderboard")

# Example usage
# handle_balance_command("player123")
# add_money("player123", 50)
# handle_pay_command("player123", "player456", 25)
# buy_item("player123", "minecraft:diamond_sword", 100)
# show_leaderboard()
