#
# Paravia
#

import random

from infrastructure import common


class Player():
    def __init__(self, name, gender):
        # player properties
        self.name = name
        self.gender = gender
        self.oldTitle = 0
        self.title = 0
        self.year = 1400
        self.difficulty = 1
        
        # Properties of the country
        self.cathedral = 0
        self.clergy = 5
        self.customsDuty = 25
        self.grainPrice = 25
        self.grainReserve = 5000
        self.incomeTax = 5
        self.isBankrupt = False
        self.isDead = False
        self.iWon = False
        self.justice = 2
        self.justiceRevenue = 0
        self.land = 10000
        self.landPrice = 10.0
        self.marketplaces = 0
        self.merchants = 25
        self.mills = 0
        self.nobles = 4
        self.palace = 0
        self.publicWorks = 1.0
        self.salesTax = 10
        self.serfs = 2000
        self.soldiers = 25
        self.ratsAte = 0
        self.treasury = 1000

        self.maleTitles = ["Sir", "Baron", "Count", "Marquis", "Duke", "Grand Duke", "Prince", "* H.R.H. King"]
        self.femaleTitles = ["Lady", "Baroness", "Countess", "Marquise", "Duchess", "Grand Duchess", "Princess", "* H.R.H. Queen"]

    # getTitle
    # get gender specific titles
    def getTitle(self):
        # make sure that the title is integer
        self.title = int(self.title)
        
        if self.gender == 'male':
            return self.maleTitles[self.title]
        else:    
            return self.femaleTitles[self.title]

    # addRevenue
    # calculate the revenue size and potential bankruptcy
    def addRevenue(self):
        self.treasury += (self.justiceRevenue + self.customsDutyRevenue)
        self.treasury += (self.incomeTaxRevenue + self.salesTaxRevenue);
        
        # Penalize deficit spending.
        if self.treasury < 0:
            self.treasury *= 1.5
            
        # Will a title make the creditors happy (for now)?
        if self.treasury < (-10000 * self.title):
            self.isBankrupt = True
   
    def attackedByNeighbor(self, opponent):
        landTaken = (self.soldiers * 1000) - (self.land / 3)
            
        if landTaken > (self.land - 5000):
            landTaken = (self.land - 5000) / 2
            
        self.land -= landTaken
        opponent.land += landTaken
        
        print("\a\n{} {} of {} invades and seizes {:2f} hectares of land!\n".format(opponent.getTitle(), opponent.name, opponent.city, landTaken))
        
        deadSoldiers = random.randint(0, 40)
        
        if deadSoldiers > (self.soldiers - 15):
            deadSoldiers = self.soldiers - 15
            
        self.soldiers -= deadSoldiers
        
        print("{} {} loses {} soldiers in battle.\n".format(self.getTitle()), self.name, deadSoldiers)


    # buyCatherdral
    # buy a cathedral
    def buyCathedral(self):
        self.cathedral += 1
        self.clergy += random.randint(0, 6)
        self.treasury -= 5000
        self.publicWorks += 1.0

    # buyCatherdral
    # buy a cathedral
    def buyGrain(self, amount):
        self.treasury -= (amount * self.grainPrice / 1000)
        self.grainReserve += amount
    

    # buyLand
    # buy land
    def buyLand(self, amount):
        self.land += amount
        self.treasury -= (amount * self.landPrice)

    
    # buyMarket
    # buy a market
    def buyMarket(self):
        self.marketplaces += 1
        self.merchants += 5
        self.treasury -= 1000
        self.publicWorks += 1.0


    # buyMill
    # buy a mill
    def buyMill(self):
        self.mills += 1
        self.treasury -= 2000
        self.publicWorks += 0.25


    # buyPalace
    # buy a palace component
    def buyPalace(self):
        self.palace = self.palace + 1
        self.nobles = self.nobles + random.randint(0, 2)
        self.treasury = self.treasury - 3000
        self.publicWorks = self.publicWorks + 0.5


    # buySoldiers
    # buy 20 soldiers
    def buySoldiers(self):
        self.soldiers = self.soldiers + 20
        self.serfs = self.serfs - 20
        self.treasury = self.treasury - 500

    
    def checkNewTitle(self):
        # Tally up our success so far.... 
        total = common.Numbers.limit10(self.marketplaces, 1)
        total += common.Numbers.limit10(self.palace, 1)
        total += common.Numbers.limit10(self.cathedral, 1)
        total += common.Numbers.limit10(self.mills, 1)
        total += common.Numbers.limit10(self.treasury, 5000)
        total += common.Numbers.limit10(self.land, 6000)
        total += common.Numbers.limit10(self.merchants, 50)
        total += common.Numbers.limit10(self.nobles, 5)
        total += common.Numbers.limit10(self.soldiers, 50)
        total += common.Numbers.limit10(self.clergy, 10)
        total += common.Numbers.limit10(self.serfs, 2000)
        total += common.Numbers.limit10(int(self.publicWorks * 100.0), 500)
        
        self.title = int(total / self.difficulty) - self.justice
        
        if self.title > 7:
            self.title = 7
            
        if self.title < 0:
            self.title = 0
            
        # Did we change (could be backwards or forwards)?
        if self.title > self.oldTitle:
            self.title = self.oldTitle + 1
            self.oldTitle = self.title
            
            if self.title >= 7:
                self.iWon = True

            print("\aGood news! " + self.name + " has achieved the rank of " + self.getTitle() + "\n\n")
            return True
        
        self.title = self.oldTitle
        return False

    
    def ratLoss(self):
        self.rats = random.randint(0, 50)
        self.ratsAte = self.grainReserve * (self.rats / 100)
        self.grainReserve = self.grainReserve - (self.ratsAte)

    
    def generateIncome(self):
        self.justiceRevenue = (self.justice * 300 - 500) * self.title
        justiceLevel = ["---","Very Fair","Moderate","Harsh","Outrageous"]
    
        revenueBase = 150.0 - float(self.salesTax - self.customsDuty - self.incomeTax)
        
        if revenueBase < 1.0:
            revenueBase = 1.0
            
        revenueBase = int(revenueBase / 100.0)
        
        self.customsDutyRevenue = self.nobles * 180 + self.clergy * 75 + self.merchants * 20 * revenueBase
        self.customsDutyRevenue += int(self.publicWorks * 100.0)
        self.customsDutyRevenue = int(float(self.customsDuty) / 100.0 * float(self.customsDutyRevenue))
    
        self.salesTaxRevenue = self.nobles * 50 + self.merchants * 25 + int(self.publicWorks * 10.0)
        self.salesTaxRevenue *= (revenueBase * (5 - self.justice) * self.salesTax)
        self.salesTaxRevenue /= 200
    
        self.incomeTaxRevenue = self.nobles * 250 + int(self.publicWorks * 20.0)
        self.incomeTaxRevenue += (10 * self.justice * self.nobles * revenueBase)
        self.incomeTaxRevenue *= self.incomeTax
        self.incomeTaxRevenue /= 100
    
        revenues = self.customsDutyRevenue + self.salesTaxRevenue + self.incomeTaxRevenue + self.justiceRevenue
        
        print("State revenues {:.0f} gold florins.\n".format(revenues))
        print("Customs Duty\tSales Tax\tIncome Tax\tJustice\n");
        print("{:>10.2f}\t{:>10.2f}\t{:>10.2f}\t{:>10.2f} ({})\n".format(self.customsDutyRevenue, self.salesTaxRevenue, self.incomeTaxRevenue, self.justiceRevenue, justiceLevel[self.justice]))

    
    def harvestLandAndGrainPrices(self):
        self.harvest = (random.randint(0, 5) + random.randint(0, 6)) / 2
        if self.harvest > 5.0:
            self.harvest = 5.0
        elif self.harvest < 1.0:
            self.harvest = 1.0

        # Generate an offset for use in later int->float conversions.
        # we are using 8 bit random numbers
        myRandom = float((random.randint(0, 32767) / 32767))
        
        # If you think this C code is ugly, you should see the original BASIC.
        workedLand = float(self.land)
        
        # available work force = number of serfs - number of serfs needed for mills (1 serf per mill) times 100
        availableWorkForce = (self.serfs - self.mills) * 100
       
        # every unit available work force can work on 5 ha of land
        processableLand = availableWorkForce * 5
        
        if processableLand < 0:
            processableLand = 0
            
        if processableLand < workedLand:
            workedLand = processableLand
        
        # to grow grain, 2 units of grain are needed per ha of land    
        maxLandToBeUsedDueToGrain = self.grainReserve * 2
        
        if maxLandToBeUsedDueToGrain < workedLand:
            workedLand = maxLandToBeUsedDueToGrain
            
        harvestPerHa = float(self.harvest + myRandom - 0.5)

        harvest = harvestPerHa * workedLand

        
        self.grainReserve = self.grainReserve + harvest
       
        # calculating grain demand
        self.grainDemand = (self.nobles * 100) + (self.cathedral * 40) + (self.merchants * 30) + (self.soldiers * 10) + (self.serfs * 5)
        
        # calculating land price
        self.landPrice = (3.0 * float(self.harvest) + float(random.randint(0, 6)) + 10.0) / 10.0
        
        if harvest < 0:
            harvest = harvest * -1
            
        if harvest < 1:
            grainDemandCoverage = 2.0
        else:
            grainDemandCoverage = self.grainDemand / harvest
            if grainDemandCoverage > 2.0:
                grainDemandCoverage = 2.0

        if grainDemandCoverage < 0.8:
            grainDemandCoverage = 0.8
            
        self.landPrice = self.landPrice * grainDemandCoverage
        
        if self.landPrice < 1.0:
            self.landPrice = 1.0
            
        self.grainPrice = 6.0 - float(self.harvest) * 3.0 + float(random.randint(0, 5)) + float(random.randint(0, 5)) * 4 * grainDemandCoverage
        if self.grainPrice < 0:
            self.grainPrice = 0.1

    
    def releaseGrain(self):
        isOK = False
        minimumGrain = float(self.grainReserve / 5)
        maximumGrain = float(self.grainReserve) - minimumGrain
        
        while isOK == False:
            print("How much grain will you release for consumption?");
            
            query = "1 = Minimum ({:.2f}), 2 = Maximum({:.2f}), or enter a value: ".format(minimumGrain, maximumGrain)
            howMuch = common.Numbers.inputInt(query)
            
            if howMuch == 1:
                howMuch = minimumGrain
                
            if howMuch == 2:
                howMuch = maximumGrain
                
            # Are we being a Scrooge?
            if (howMuch + 1) < minimumGrain:
                print("You must release at least 20 % of your reserves.")
                
            # Whoa. Slow down there son. */
            elif (howMuch - 1) > maximumGrain:
                print("You must keep at least 20%.")
                
            else:
                isOK = True

        self.soldierPay = 0
        self.marketRevenue = 0
        self.newSerfs = 0
        self.deadSerfs = 0
        self.transplantedSerfs = 0
        self.fleeingSerfs = 0
        
        self.invadeMe = False
        
        self.grainReserve = self.grainReserve - howMuch
        
        demandSatisfaction = float(howMuch) / float(self.grainDemand - 1.0)
        
        if demandSatisfaction > 0.0:
            demandSatisfaction = demandSatisfaction / 2.0
            
        if demandSatisfaction > 0.25:
            demandSatisfaction = demandSatisfaction / 10.0 + 0.25
            
        zp = 50.0 - float(self.customsDuty) - float(self.salesTax) - float(self.incomeTax)
        
        if zp < 0.0:
            zp = zp * float(self.justice)
            
        zp = zp / 10.0
        
        if zp > 0.0:
            zp = zp + 3.0 - float(self.justice)
                       
        demandSatisfaction = demandSatisfaction + (zp / 10.0)
        
        if demandSatisfaction > 0.5:
            demandSatisfaction = 0.5
            
        if howMuch < (self.grainDemand - 1):
            x = float(self.grainDemand - howMuch) / float(self.grainDemand) * 100.0 - 9.0
            
            xp = float(x)
        
            if x > 65.0:
                x = 65.0
            
            if x < 0.0:
                xp = 0.0
                x = 0.0
            
            self.serfsProcreating(3.0)
            self.serfsDecomposing(xp + 8.0)
        else:
            self.serfsProcreating(7.0)
            self.serfsDecomposing(3.0)
            
            if (self.customsDuty + self.salesTax) < 35:
                self.merchants += random.randint(0, 4)
                
            if self.incomeTax < random.randint(0, 28):
                self.nobles += random.randint(0, 2)
                self.clergy += random.randint(0, 3)

            if howMuch > (int)(float(self.grainDemand * 1.3)):
                zp = float(self.serfs / 1000.0)
                z = float(howMuch - self.grainDemand) / (float(self.grainDemand * 10.0)) * float(zp) * float(random.randint(0, 25)) + float(random.randint(0, 40))
                self.transplantedSerfs = int(z)
                self.serfs = self.serfs + self.transplantedSerfs
                
                print("{} serfs move to the city".format(self.transplantedSerfs))
                
                zp = float(z)
                z = float(zp) * random.random()

                if z > 50.0:
                    z = 50.0
                    
                self.merchants = self.merchants + int(z)
                self.nobles = self.nobles + 1
                self.clergy = self.clergy + 2

        if self.justice > 2:
            self.justiceRevenue = self.serfs / 100 * (self.justice - 2) * (self.justice - 2)
            self.justiceRevenue = random.randint(0,self.justiceRevenue)
            self.serfs -= self.justiceRevenue
            self.fleeingSerfs = self.justiceRevenue
            print("{} serfs flee harsh justice\n".format(self.fleeingSerfs))

        self.marketRevenue = self.marketplaces * 75;
        
        if self.marketRevenue > 0:
            self.treasury  += self.marketRevenue
            print("Your market earned {} florins.".format(self.marketRevenue))

        self.millRevenue = self.mills * (55 + random.randint(0,250))
        
        if self.millRevenue > 0:
            self.treasury += self.millRevenue
            print("Your woolen mill earned  {} florins.".format(self.millRevenue))

        self.soldierPay = self.soldiers * 3
        self.treasury -= self.soldierPay
        
        print("You paid your soldiers {} florins.".format(self.soldierPay))
        print("You have {} serfs in your city.".format(self.serfs))
        input("(Press ENTER): ")
        
        if (self.land / 1000) > self.soldiers:
            self.invadeMe = True

        if (self.land / 500) > self.soldiers:
            self.invadeMe = True


    # seizing assets from a bankrupt player.   
    def seizeAssets(self):
        self.marketplaces = 0
        self.palace = 0
        self.cathedral = 0
        self.mills = 0
        self.land = 6000
        self.publicWorks = 1.0
        self.treasury = 100
        self.isBankrupt = False

    
    def sellGrain(self):
        howMuch = common.Numbers.inputInt("How much grain do you want to sell? ")
        if howMuch > self.grainReserve:
            print("You don't have it.")
            return

        self.treasury += (howMuch * self.grainPrice / 1000)
        self.grainReserve -= howMuch

    
    def sellLand(self, amount):
        if amount > (self.land - 5000):
            print("You can't sell that much")
            return

        self.land -= amount
        self.treasury += (amount * self.landPrice)

        
    def serfsDecomposing(self, decomposingBase):
        #split decomposingBase into the part before and after the decimal
        absc = int(decomposingBase)
        ordx = decomposingBase - float(absc)
        
        self.deadSerfs = int((float(random.randint(0, absc) + ordx)) * (float(self.serfs) / 100.0))
        self.serfs -= self.deadSerfs
        
        print("{0:.0f} serfs died this year.\n".format(self.deadSerfs))

    
    def serfsProcreating(self, procreationBase):
        #split procreationBase into the part before and after the decimal
        absc = int(procreationBase)
        ordx = procreationBase - float(absc)
        
        self.newSerfs = int((float(random.randint(0, absc) + ordx)) * (float(self.serfs) / 100.0))
        self.serfs += self.newSerfs
        
        print("{0:.0f} serfs born this year.\n".format(self.newSerfs))
    
    
    def adjustTax(self):
        taxNumber = ''
        while taxNumber != 'q' :
            print("\n"+ self.getTitle() + " " + self.name)

            self.generateIncome()
            
            print("({:>10.2f}%)\t({:>10.2f}%)\t({:>10.2f}%)".format(self.customsDuty, self.salesTax, self.incomeTax))
            print("\n1. Customs Duty, 2. Sales Tax, 3. Wealth Tax, 4. Justice\n")
            taxNumber = input("Enter tax number for changes, q to continue: ")
            if taxNumber == '1':
                duty = common.Numbers.inputInt("New customs duty (0 to 100): ")
                if duty > 100:
                    duty = 100
                if duty < 0:
                    duty = 0
                self.customsDuty = duty
            elif taxNumber == '2':
                duty = common.Numbers.inputInt("New sales tax (0 to 50): ")
                if duty > 50:
                    duty = 50
                if duty < 0:
                    duty = 0
                self.salesTax = duty
            elif taxNumber == '3':
                duty = common.Numbers.inputInt("New wealth tax (0 to 25): ")
                if duty > 25:
                    duty = 25
                if duty < 0:
                    duty = 0
                self.incomeTax = duty;
                break;
            elif taxNumber == '4':
                duty = common.Numbers.inputInt("Justice: 1. Very fair, 2. Moderate, 3. Harsh, 4. Outrageous: ");
                try:
                    if duty > 4:
                        duty = 4
                    if duty < 1:
                        duty = 1
                    self.justice = duty;
                except ValueError:
                    print("You did not enter correctly. The value is not changed")
        
        self.addRevenue()
        
        if self.isBankrupt == True:
            self.seizeAssets()

    
    def imDead(self):
        pass
