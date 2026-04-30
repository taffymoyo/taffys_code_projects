import classes

print("Welcome to My RPG")
name = input("\nChoose a name for your character: ")
print("Now select your class")
player_input = input("\n1 - Melee: \n2 - Ranger: \n3 - Mage: \nType your answer ")
while True:
    if player_input == "1":
        print("\nYou selected the melee class!")
        char = classes.Melee(name)
        default_weapon = classes.Battered_Gloves()
        char.weapons.append(default_weapon)
        char.weapon = default_weapon
        print(f"Your weapon is {default_weapon.name}")
        break

    elif player_input == "2":
        print("\nYou selected the ranger class!")
        char = classes.Ranger(name)
        default_weapon = classes.Wooden_Bow()
        char.weapons.append(default_weapon)
        char.weapon = default_weapon
        print(f"Your weapon is {default_weapon.name}")
        break

    elif player_input == "3":
        print("\nYou selected the mage class!")
        char = classes.Mage(name)
        default_weapon = classes.Dusty_Grimoire()
        char.weapons.append(default_weapon)
        char.weapon = default_weapon
        print(f"Your weapon is {default_weapon.name}")
        break
    
    else:
        print("Please selected a valid class, 1, 2 or 3")
        player_input = input("\n1 - Melee: \n2 - Ranger: \n3 - Mage: \n Type your answer ")

print("\nNow lets begin your journey, practice on this enemy!")
enemy = classes.Enemy("enemy")
enemies = [enemy]

print(f"\nYou have {char.hp} hitpoints and your enemy has {enemy.hp} hitpoints, make sure not to lose all of yours")

# first battle sequence
battle = classes.Battle(char, enemies)
battle.current_battle(char, enemies)


print("\n#\n")
print("A strange symbol hovers infront of you...")

print(f"\nCongratulations on killing the enemy, but you only have {char.hp} hitpoints left, lets get you something to heal up!")
print("\nHere you'll be able to see your inventory during battles, lets take a look now")
print(f"\n {list(char.inventory.keys())} \n")

while True:
    if len(char.inventory) == 0:
        char.inventory = {"Mushroom": classes.Healing_Item("Mushroom", 20, 1)}

    player_input = input("Seems like you have a mushroom! Select 1 to use it: ")
    if player_input == "1":
        char.inventory["Mushroom"].heal(char)
        del char.inventory["Mushroom"]
        break
    else:
        print("\nPlease select 1 to use the mushroom")

print(f"\nNice, you just healed up to {char.hp} hitpoints, now you're ready to continue\n")
print(f'{char.inventory} Now your inventory is empty, make sure to find healing items on the way.')

print("\nYou stumbled across a shiny object, would you like to pick it up?\n")
player_input = input("1 - Yes: \n2 - No \n")

while True:
    if player_input == "1":
        enemies = [classes.Enemy(f"Enemy{i+1}") for i in range(2)]

        battle = classes.Battle(char, enemies)
        battle.current_battle(char, enemies)
        
        print("\nCongratulations, you won another battle")
        print("\n@\n")
        print("A strange symbol hovers infront of you...\n")

        # Weapon Upgrades
        if isinstance(char, classes.Melee):
            second_weapon = classes.Iron_Gauntlets()
        elif isinstance(char, classes.Ranger):
            second_weapon = classes.Diamond_Bow()
        elif isinstance(char, classes.Mage):
            second_weapon = classes.Gemstone_Grimoire()
        
        char.weapons.append(second_weapon)

        print("Would you like to equip the new weapon?")
        player_input = input("1 - Yes: \n2 - No \nYour input: ")

        if player_input == "1":
            char.weapon = second_weapon
            print("---------------------------------------------------------")
            print(f"You unlocked {second_weapon.name} all attacks buffed by {second_weapon.buff}x")
            print("---------------------------------------------------------\n")
            break

        elif player_input == "2":
            print("\nYou chose to leave the weapon (why would you do that lol)")
            break

    elif player_input == "2":
        print("You chose not to pick up the object")
        break

    else:
        print("\nPlease select a valid option\n")

print()
print("You found a healing potion, would you like to pick it up?")
healing_potion = classes.Healing_Item("healing_potion", 50)
char.pick_up_healing_item(healing_potion)

print("\n!\n")
print("A strange symbol hovers infront of you...")

print("You see a massive castle in the distance, and walk torwards it")
print("\nThe gate has a puzzle on it, you must complete it to enter\n")

tries = 3
while True:
    player_input = input("Enter the code: ")
    if player_input == "#@!":
        break
    else:
        if tries > 0:
            print(f"Wrong code! Try again...")
        else:
            print("You suddenly remember one of the strange symbols from earlier\n#\n")
        if tries > 0:
            tries -= 1

print("The gates of castle open!")

print("\nYou find a chest in the castle with a healing potion, would you like to pick it up?")
healing_potion = classes.Healing_Item("healing_potion", 50)
char.pick_up_healing_item(healing_potion)

print("\nYou hear a strange noise in the distance, you decide to go investigate")
boss = classes.Boss("boss")
print("\nYou see a a giant figure as you approach...")
print("\n\nQuick you have 3 seconds to dodge his attack!\n\n")
boss.charge(char)

enemies = [boss]
battle = classes.Battle(char, enemies)


print("\nYou have found yourself in a battle with the castle guardian!\n")
battle.current_battle(char, enemies)

print("Congratulations, you killed the boss and found the treasure!") 
print()
print("---------------------------------------------------------\n")
print("GAME OVER") 
print("\n---------------------------------------------------------")