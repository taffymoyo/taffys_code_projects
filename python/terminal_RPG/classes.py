import time
import random


# Player Classes
class Player:
    def __init__(self, name):
        self.name = name
        self.hp = 100
        self.defence = 1
        self.inventory = {"Mushroom": Healing_Item("Mushroom", 20, 1)}
        self.attacks  = [("Base Attack", self.base_attack)] # should i change to self.actions to allow for a heal
        self.weapons = []
        self.weapon = None
        self.max_hp = 100
    
    def base_attack(self, target):
        """
        target loses 20 hp after being attacked assuming target has self.hp
        """
        target = target[0]
        damage = 20 // target.defence
        target.hp -= damage * self.weapon.buff
        print(f"You did {damage} damage! Enemy HP: {target.hp if target.hp > 0 else 0}")

    def pick_up_healing_item(self, healing_item):
        while True:
            player_input = input("1 - Yes: \n2 - No \nYour choice: ")
            if player_input == "1":
                if healing_item.name in self.inventory:
                    self.inventory[healing_item.name].quantity += 1
                else:
                    self.inventory[healing_item.name] = healing_item
                print(f"\nWould you like to use it now, you currently have {self.hp} hitpoints?")

                while True:
                    new_input = input("1 - Yes: \n2 - No \nYour choice: ")
                    if new_input == "1":
                        self.inventory[healing_item.name].heal(self)
                        self.inventory[healing_item.name].quantity -= 1
                        print(f"\nYou healed for {healing_item.value} hp, now you have {self.hp} hitpoints")

                        if self.inventory[healing_item.name].quantity == 0:
                            del self.inventory[healing_item.name]

                        break

                    elif new_input == "2":
                        print(f"You keep the healing potion in your inventory\n")
                        break

                    else:
                        print("Please pick either 1 or 2")
                    
                if len(self.inventory) > 0:
                    for name, item in self.inventory.items():
                        print(f"You currently have {item.quantity} {name}s in your inventory")
                else:
                    print("\nYour inventory is currently empty")

                break

            elif player_input == "2":
                print("You chose to leave the healing potion behind")
                break
            
            else:
                print("Please pick either 1 or 2")
        
    def use_healing_item(self, used=False):

        if len(self.inventory) == 0:
            print("You dont have any healing items")
            return
        
        while True:
            print("Choose a healing item")
            items = list(self.inventory.items())

            for i, (name, item) in enumerate(items):
                print(f"{i + 1} - {name}: {item.quantity}")
            
            print(f"{i+2} - Back")

            player_choice = int(input("Your choice: ")) - 1
            print()

            if 0 <= player_choice < len(items):
                name, item = items[player_choice]
                item.heal(self)
                item.quantity -= 1

                if item.quantity == 0:
                    del self.inventory[name]

                break

            elif player_choice == len(items):
                print("You return back to the attack menu\n")
                return
            
            else:
                print("Please select a valid option\n")

        used = True
        return used

class Melee(Player):
    def __init__(self, name):
        super().__init__(name)
        self.defence = 1.2
        self.attacks.append(("Gauntlet Flurry", self.gauntlet_flurry))
    
    def gauntlet_flurry(self, target):
        """
        target loses 30 hp after being attacked assuming target has self.hp
        """
        target = target[0]
        damage = 30 // target.defence
        target.hp -= damage * self.weapon.buff
        print(f"You did {damage} damage! Enemy HP: {target.hp if target.hp > 0 else 0}")

class Ranger(Player):
    def __init__(self, name):
        super().__init__(name)
        self.air = True
        self.attacks.append(("Storming_Arrows", self.storming_arrows))
    
    def storming_arrows(self, targets):
        """
        target loses 30 hp after being attacked assuming target has self.hp
        """
        for i, target in enumerate(targets):
            damage = 15 / target.defence
            target.hp -= damage * self.weapon.buff
            print(f"You did {damage} damage! Enemy {i+1} HP: {target.hp if target.hp > 0 else 0}")

class Mage(Player):
    def __init__(self, name):
        super().__init__(name)
        self.attacks.append(("Fireball", self.fireball))

    def fireball(self, target):
        target = target[0]
        damage = (10 * self.weapon.buff) // target.defence
        target.hp -= damage * self.weapon.buff
        target.status = {"burn": [20, 3]}
        print(f"You did {damage / target.defence} damage! Enemy HP: {target.hp if target.hp > 0 else 0}")

# Enemy Classes
class Enemy:
    def __init__(self, name):
        self.name = name
        self.hp = 50
        self.defence = 1
        self.attacks = [("Base Attack", self.base_attack)]
        self.status = {}

    
    def base_attack(self, target):
        """
        target loses 20 hp after being attacked assuming target has self.hp
        """
        damage = 10 // target.defence
        target.hp -= damage
        print(f"{self.name} used base attack and did {damage} damage! Your HP: {target.hp}")

class Boss(Enemy):
    def __init__(self, name):
        super().__init__(name)
        self.hp = 100
        self.defence = 1.25
        self.attacks.extend([("Thundering Smash", self.thundering_smash), 
                             ("Charge", self.charge)])

    def thundering_smash(self, target):
        damage = 30 // target.defence
        target.hp -= damage
        print(f"{self.name} used thundering smash and did {damage} damage! Your HP: {target.hp}")
    
    def charge(self, target):

        print(f"{self.name} is preparing an attack! get ready to dodge by clicking x")
        delay = random.randint(3,5)
        time.sleep(delay)
        old_input = time.time()
        print("\nCLICK x TO DODGE\n")
        player_input = input("")

        new_input = time.time()
        reaction_time = new_input - old_input
        if reaction_time < 2 and player_input.lower() == "x":
            print("You successfully dodged the attack")
        else:
            original = target.hp
            target.hp-= 50 * target.defence
            print(f"You didnt dodge in. time and took {original - target.hp} damage")


# Item Classes
class Item:
    def __init__(self, name, value, quantity = 1):
        self.name = name
        self.value = value
        self.quantity = 1

class Healing_Item(Item):
    def __init__(self, name, value, quantity=1):
        super().__init__(name, value, quantity)
    
    def heal(self, target):
            if self.quantity > 0:
                target.hp += self.value
                if target.hp > target.max_hp:
                    target.hp = target.max_hp
                print(f"{target.name} healed for {self.value} HP!")

class Battle:
    def __init__(self, player, enemies):
        self.player = player
        self.enemies = enemies
    
    def player_turn(self):
        used = False

        while not used:
            # show attacks
            print("Choose an action!")
            for i, (name, _) in enumerate(self.player.attacks):
                print(f"{i+1} - {name}")
            
            print(f"{len(self.player.attacks)+1} - Heal")
            
            choice = int(input("Your Choice: ")) - 1
            print()
            if 0 <= choice < len(self.player.attacks):
                # call the attack in one line
                self.player.attacks[choice][1](self.enemies)  # single-target for now
                used = True
            
            # healing option
            elif choice == len(self.player.attacks):
                used = self.player.use_healing_item()
            
            else:
                print("Please select a valid option\n")
        
            # status effect
        for enemy in self.enemies:
            if len(enemy.status) > 0:
                for name, value in list(enemy.status.items()):
                    enemy.hp -= value[0] / enemy.defence  # subtracting debuff
                    value[1] -= 1 # decreasing rounds
                    print(f"Enemy currently effect by {name} debuff and took {value[0] / enemy.defence } damage, the debuff will last for {value[1]} more rounds")
                    if value[1] == 0: 
                        del enemy.status[name]

            
    def enemies_turn(self):
        for enemy in self.enemies:
            if enemy.hp > 0:
                attack_index = random.randint(0, len(enemy.attacks) - 1)
                enemy.attacks[attack_index][1](self.player)
    
    def current_battle(self, player, enemies):
        print(f'\nYou have entered a battle with {len(enemies)} enemies')
        while enemies:
            print("---------------------------------------------------------\n")
            self.player_turn()
            print("---------------------------------------------------------")
            time.sleep(2)
            self.enemies_turn()
            print("---------------------------------------------------------\n")
            time.sleep(2)

            enemies[:] = [enemy for enemy in enemies if enemy.hp > 0]
            print(f'{len(enemies)} remain: {", ".join(enemy.name for enemy in enemies)}')
            if player.hp <= 0:
                print("You died! Better luck next time")
                quit()
                
            print(f"\n-> You have {player.hp} hitpoints <-\n")

            print("ENEMIES")
            for enemy in enemies:
                print(f'{enemy.name} has {enemy.hp} hitpoints!')
            print()

        print("Congratulations on winning the battle")
            


class Weapon:
    def __init__(self, name, buff):
        self.name = name
        self.buff = buff

class Gloves(Weapon):
    def __init__(self, name, buff):
        super().__init__(name, buff)

class Battered_Gloves(Gloves):
    def __init__(self):
        super().__init__("Battered Gloves", buff = 1)

class Iron_Gauntlets(Gloves):
    def __init__(self):
        super().__init__("Iron Gauntlets", buff = 1.25)

class Bow(Weapon):
    def __init__(self, name, buff):
        super().__init__(name, buff)

class Wooden_Bow(Bow):
    def __init__(self):
        super().__init__("Battered Gloves", buff = 1)

class Diamond_Bow(Bow):
    def __init__(self):
        super().__init__("Diamond Bow", buff = 1.25)

class Grimoire(Weapon):
    def __init__(self, name, buff):
        super().__init__(name, buff)

class Dusty_Grimoire(Grimoire):
    def __init__(self):
        super().__init__("Dusty Grimoire", 1)

class Gemstone_Grimoire(Grimoire):
    def __init__(self):
        super().__init__("Gemstone Grimoire", 1.25)


# maybe add attack cooldonw system