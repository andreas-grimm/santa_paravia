#
# paravia.py
#
# /******************************************************************************
# **                                                                          **
# ** Santa Paravia & Fiumaccio. Translated from the original TRS-80 BASIC     **
# ** source code into C by Thomas Knox <tknox@mac.com>.                       **
# ** adopted to Python by Andreas Grimm <andreas.grimm@gricom.eu              **
# **                                                                          **
# ** Original program (C) 1979 by George Blank                                **
# ** <gwblank@postoffice.worldnet.att.net>                                    **
# **                                                                          **
# ******************************************************************************
#
# Copyright (C) 2000 Thomas Knox
#
# Portions Copyright (C) 1979 by George Blank, used with permission. This program is 
# free software; you can redistribute it and/or modify it under the terms of the 
# GNU General Public License as published by the Free Software Foundation; either version 2
# of the License, or (at your option) any later version. This program is distributed 
# in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the 
# implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
# GNU General Public License for more details. You should have received a copy of 
# the GNU General Public License along with this program; if not, write to the Free 
# Software Foundation, Inc., 59 Temple Place - Suite 330, Boston, MA 02111-1307, USA.
# Thomas Knox
# tknox@mac.com

import random

from gameClasses import player
from infrastructure import common

cityList = ["Santa Paravia", "Fiumaccio", "Torricella", "Molinetto", "Fontanile", "Romanga", "Monterana"]
players = []


def instructions():
    print("Santa Paravia and Fiumaccio\n")
    print(
        "You are the ruler of a 15th century Italian city state. If you rule well, you will receive higher titles. The")
    print(
        "first player to become king or queen wins. Life expectancy then was brief, so you may not live long enough to win.")
    print(
        "The computer will draw a map of your state. The size of the area in the wall grows as you buy more land. The")
    print(
        "size of the guard tower in the upper left corner shows the adequacy of your defenses. If it shrinks, equip more")
    print(
        "soldiers! If the horse and plowman is touching the top of the wall, all your land is in production. Otherwise you need more")
    print(
        "serfs, who will migrate to your state if you distribute more grain than the minimum demand. If you distribute less")
    print(
        "grain, some of your people will starve, and you will have a high death rate. High taxes raise money, but slow down")
    input("economic growth.\n\n(Press ENTER to begin game)")


def playGame():
    allDead = False

    evilBaron = player.Player("Peppone", "male")

    while True:
        for currentPlayer in players:
            if currentPlayer.isDead == False:
                newTurn(currentPlayer=currentPlayer, evilBaron=evilBaron)

        allDead = True;
        for currentPlayer in players:
            if allDead == True and currentPlayer.isDead == False:
                allDead = False

        for currentPlayer in players:
            if currentPlayer.iWon == True:
                print("Game Over. " + currentPlayer.getTitle() + " " + currentPlayer.name + " wins.")
                return

        if allDead == True:
            print("The game has ended.\n");
            break


def isDead(currentPlayer):
    print("\n\nVery sad news.\n" + currentPlayer.title + currentPlayer.name + " has just died")

    if currentPlayer.year > 1450:
        print("of old age after a long reign.\n")
    else:
        why = random.randint(0, 8)
        deathReason = {
            0: "",
            1: "",
            2: "",
            3: "of pneumonia after a cold winter in a drafty castle.",
            4: "of typhoid after drinking contaminated water.",
            5: "in a smallpox epidemic.",
            6: "after being attacked by robbers while travelling.",
            7: "",
            8: "of food poisoning."
        }
        print(deathReason[why])

    currentPlayer.isDead = True

    input("\n(Press ENTER): ")


def printGrain(harvest):
    harvestRating = {
        1: "Drought. Famine Threatens. ",
        2: "Bad Weather. Poor Harvest. ",
        3: "Normal Weather. Average Harvest. ",
        4: "Good Weather. Fine Harvest. ",
        5: "Excellent Weather. Great Harvest! "
    }
    print(harvestRating[int(harvest)])


def buySellGrain(currentPlayer):
    finished = False
    while (finished == False):
        print("\nYear " + str(currentPlayer.year))
        print("\n" + currentPlayer.getTitle() + " " + currentPlayer.name)
        print("\nRats ate " + str(currentPlayer.rats) + "% of your grain reserves. ({0:.2f} steres)".format(
            currentPlayer.ratsAte))
        printGrain(currentPlayer.harvest)
        print("\nGrain\t\tGrain\t\tPrice of\tPrice of\tTreasury")
        print("Reserve\t\tDemand\t\tGrain\t\tLand\n")
        print(" {0:>10.2f}\t{1:>10.2f}\t{2:>10.2f}\t\t{3:>4.2f}\t\t{4:>4.2f}\n".format(currentPlayer.grainReserve,
                                                                                       currentPlayer.grainDemand,
                                                                                       currentPlayer.grainPrice,
                                                                                       currentPlayer.landPrice,
                                                                                       currentPlayer.treasury))
        print("steres\t\tsteres\t\t1000 st.\thectare\t\tgold florins\n")
        print("\nYou have " + str(currentPlayer.land) + " hectares of land.")
        print("\n1. Buy grain, 2. Sell grain, 3. Buy land, 4. Sell land ")

        select = input("(Enter q to continue):")
        if select == 'q':
            finished = True

        if select == '1':
            amount = common.Numbers.inputInt("How much grain do you want to buy (0 to specify a total)? ")
            if amount == 0:
                amount = int(input("How much total grain do you wish? "))
                amount = amount - currentPlayer.grainReserve
            if amount < 0:
                print("Invalid total amount.\n")
                return
            currentPlayer.buyGrain(amount)

        if select == '2':
            amount = common.Numbers.inputInt("How much grain do you want to sell (0 to specify a total)? ")
            if amount == 0:
                amount = int(input("How much total grain do you wish? "))
                amount = currentPlayer.grainReserve - amount
            if amount < 0:
                print("Invalid total amount.\n")
                return
            currentPlayer.sellGrain(amount)

        if select == '3':
            amount = common.Numbers.inputInt("How much land do you want to buy (0 to specify a total)? ")
            if amount == 0:
                amount = int(input("How much total land do you wish? "))
                amount = amount - currentPlayer.land
            if amount < 0:
                print("Invalid total amount.\n")
                return
            currentPlayer.buyLand(amount)

        if select == '4':
            amount = common.Numbers.inputInt("How much land do you want to sell (0 to specify a total)? ")
            if amount == 0:
                amount = int(input("How much total land do you wish? "))
                amount = currentPlayer.land - amount
            if amount < 0:
                print("Invalid total amount.\n")
                return
            currentPlayer.sellLand(amount)


def showStats():
    print("    Nobles\t   Soldiers\t    Clergy\t Merchants\t     Serfs\t      Land\t  Treasury\n")
    for x in players:
        print("\n" + x.getTitle() + " " + x.name)
        print("{0:>10.2f}\t{1:>10.2f}\t{2:>10.0f}\t{3:>10.0f}\t{4:10.0f}\t{5:10.0f}\t{6:>10.2f}\n".format(x.nobles,
                                                                                                          x.soldiers,
                                                                                                          x.clergy,
                                                                                                          x.merchants,
                                                                                                          x.serfs,
                                                                                                          x.land,
                                                                                                          x.treasury))
        input("\n(Press ENTER): ")


def statePurchases(currentPlayer):
    choice = ""
    while choice != 'q':
        print("\n\n" + currentPlayer.getTitle() + " " + currentPlayer.name + "\nState purchases.")
        print("1. Marketplace ({0:.2f})\t\t\t\t1000 florins\n".format(currentPlayer.marketplaces))
        print("2. Woolen mill ({0:.2f})\t\t\t\t2000 florins\n".format(currentPlayer.mills))
        print("3. Palace (partial) ({0:.2f})\t\t\t3000 florins\n".format(currentPlayer.palace))
        print("4. Cathedral (partial) ({0:.2f})\t\t\t5000 florins\n".format(currentPlayer.cathedral))
        print("5. Equip one platoon of serfs as soldiers\t500 florins\n")
        print("\nYou have {0:.2f} gold florins.\n".format(currentPlayer.treasury))
        print("\nTo continue, enter q. To compare standings, enter 6\n");
        choice = input("Your choice: ")
        if choice == "1":
            currentPlayer.buyMarket()
        elif choice == "2":
            currentPlayer.buyMill()
        elif choice == "3":
            currentPlayer.buyPalace()
        elif choice == "4":
            currentPlayer.buyCathedral()
        elif choice == "5":
            currentPlayer.buySoldiers()
        elif choice == "6":
            showStats()


def newTurn(currentPlayer, evilBaron):
    currentPlayer.harvestLandAndGrainPrices()
    currentPlayer.ratLoss()
    buySellGrain(currentPlayer)
    currentPlayer.releaseGrain()

    attacked = False
    if currentPlayer.invadeMe == True:
        for opponent in players:
            if opponent != currentPlayer:
                if opponent.soldiers > int(currentPlayer.soldiers * 2.4):
                    currentPlayer.attackedByNeighbor(opponent)
                    attacked = True

        if attacked != True:
            currentPlayer.attackedByNeighbor(evilBaron)

    currentPlayer.adjustTax()

    # if the player is bankrupt, seize the assets
    if currentPlayer.isBankrupt == True:
        currentPlayer.seizeAssets()
        print("\n" + + currentPlayer.getTitle() + " " + currentPlayer.name + " is bankrupt.")
        print("\nCreditors have seized much of your assets.")
        input("(Press ENTER): ")

    # DrawMap(Me);
    statePurchases(currentPlayer)
    currentPlayer.checkNewTitle()

    currentPlayer.year = currentPlayer.year + 1

    # if currentPlayer.year == currentPlayer.yearOfDeath:
    #    currentPlayer.imDead()
    if currentPlayer.title >= 7:
        currentPlayer.iWon = True


def main():
    try:
        # Start the game
        print("Santa Paravia and Fiumaccio\n\n")
        answer = input("Do you wish instructions (Y or N)? ")

        if answer == 'y' or answer == 'Y':
            instructions()

        numOfPlayers = int(input("How many people want to play (1 to 6)? "))

        if numOfPlayers < 1 or numOfPlayers > 6:
            print("Thanks for playing.\n")
            return

        print("\nWhat will be the difficulty of this game:\n\n1. Apprentice")
        print("2. Journeyman\n3. Master\n4. Grand Master\n")
        level = int(input("Choose: "))

        if level < 1:
            level = 1

        if level > 4:
            level = 4

        for counter in range(numOfPlayers):
            name = input("Who is the ruler of " + cityList[counter] + "? ")
            name = name.rstrip()

            genderAnswer = input("Is " + name + " a man or a woman (M or F)? ")
            if genderAnswer == 'm' or genderAnswer == 'M':
                gender = "male"
            else:
                gender = "female"

            currentPlayer = player.Player(name, gender)
            currentPlayer.difficulty = level
            players.append(currentPlayer)

        # Enter the main game loop.
        playGame()

    except KeyboardInterrupt:
        print("\nGame terminated by player")

    # We're finished.
    return


main()
