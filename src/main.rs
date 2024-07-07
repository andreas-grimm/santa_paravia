/*
  Name: paravia.rs
  Description:This is a port of the original TRS-80 BASIC code for Santa Paravia and Fiumaccio, 
  (C) 1979 George Blank (used with permission).
  By: Thomas Knox                                                               

  Inputs:N/A                                                                    

  Returns:N/A                                                                   

  Assumes:Should compile and run on any system with an Rust compiler.         

  Side Effects:N/A                                                              

  This code is copyrighted and has limited warranties.

  **
  **                                                                           
  ** Santa Paravia & Fiumaccio. Translated from the original TRS-80 BASIC 
  **      
  ** source code into C by Thomas Knox <tknox@mac.com>.
  **                         
  ** 
  **                                                                           
  ** Original program (C) 1979 by George Blank
  **                                  
  ** <gwblank@postoffice.worldnet.att.net>
  **                                      
  **

  Copyright (C) 2000 Thomas Knox                                                  
  Portions Copyright (C) 1979 by George Blank, used with permission.              
  This program is free software; you can redistribute it and/or                   
  modify it under the terms of the GNU General Public License                     
  as published by the Free Software Foundation; either version 2                  
  of the License, or (at your option) any later version.                          
  This program is distributed in the hope that it will be useful,                 
  but WITHOUT ANY WARRANTY; without even the implied warranty of                  
  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the                    
  GNU General Public License for more details.                                    
  You should have received a copy of the GNU General Public License               
  along with this program; if not, write to the Free Software                     
  Foundation, Inc., 59 Temple Place - Suite 330, Boston, MA 02111-1307, USA.      
  Thomas Knox                                                                     
  tknox@mac.com  

*/
mod player;
use crate::player::Player;
use text_io::read;

fn main() {

  /* Declare constants */
    /* Cities */
    let city_list = vec!["Monterana", "Santa Paravia", "Fiumaccio", "Torricella", "Molinetto", "Fontanile", "Romanga" ];

    // define a vector of players (0..6)
    let mut players: Vec<Player>  = Vec::new();

    println!("Santa Paravia and Fiumaccio\n\n");
    print!("Do you wish instructions (Y or N)? ");

    let mut answer: String = read!("{}\n");

    let answer_char = answer.chars().next().unwrap();
    if (answer_char == 'y') || (answer_char == 'Y') {
        rules();
    }

    print!("How many people want to play (1 to 6)? ");
    answer = read!("{}\n");

    let num_of_players = answer.chars().next().expect("Incorrect input").to_digit(10).unwrap_or(0);

    if (num_of_players < 1) || (num_of_players > 6) {
        println!("Thanks for playing.\n");
        return
    }

    println!("\nWhat will be the difficulty of this game:\n\n1. Apprentice");
    println!("2. Journeyman\n3. Master\n4. Grand Master\n");
    print!("Choose: ");
    answer = read!("{}\n");

    let mut level = answer.chars().next().expect("Incorrect input").to_digit(10).unwrap_or(0);

    if level < 1 {
        level = 1;
    }

    if level > 4 {
        level = 4;
    }

    for counter in 1..=num_of_players {
        print!("Player #{}, how do you want to name your country [{}] ? ",counter, city_list[counter as usize]);
        answer = read!("{}\n");
        let mut city_name = answer.clone();
        if city_name.len() < 1 {
            city_name = city_list[counter as usize].parse().unwrap();
        }

        print!("Who is the ruler of {}? ",city_name);
        answer = read!("{}\n");
        let player_name = answer.clone();

        print!("\nIs {} a man or a woman (M or F)? ", player_name);
        answer = read!("{}\n");
        let answer_char = answer.chars().next().unwrap();
        let mut gender :bool;
        if (answer_char == 'f') || (answer_char == 'F') {
            gender = false;
        } else {
            gender = true;
        }

    // create a new player and push it in the vector
        let mut player :Player = Player::new();
        player = player.clone().init(player_name, gender, city_name);

        println!("Thank you, {} {} of {}\n\n", player.clone().get_title(), player.clone().get_name(), player.clone().get_city());

        players.push(player);
    }

    // Enter the main game loop.
    play_game(players.clone());

    // We're finished.
    return
}


fn play_game(mut players: Vec<Player>) {
    let all_dead = false;
    let mut all_dead = true;

    loop {
        for mut counter in 0..players.len() {
            let mut player= players[counter].clone();

            if player.clone().dead() == false {
                player = new_turn(player, players.clone());
            }

            if player.clone().dead() == false {
                all_dead = false;
            }

            if player.clone().i_won() == true {
                println!("Game Over. {} {} wins...", player.clone().get_title(), player.clone().get_name());
                return
            }

            // replace the old player
            players[counter] = player;

        }

        if all_dead == true {
            println!("The game has ended.\n");
            break;
        }
    }
}

fn new_turn(mut player: Player, mut players: Vec<Player>) -> (Player) {
    // Step 1: Calculate harvest and loss of grain due to rats
    player = player.harvest_land_and_grain_prices();
    player = player.rat_loss();

    // Step 2: Adjust grain and land and release food to people
    player = buy_and_sell_grain(player);
    player = release_grain(player);

    // Step 3: Verify military defense capabilities
    let mut attacked = false;

    if player.clone().get_invade_me() == true {
        let mut dead_soldiers = 0;
        let mut land_taken = 0;;
        // let's see whether one of the other players is strong enough to attack
        for mut counter in 0..players.len() {
            let mut opponent= players[counter].clone();
            //I cannot attack myself
            //TODO Thing about civil unrest...
            if opponent.clone().get_name() != player.clone().get_name() {
                if opponent.clone().get_soldiers() > (player.clone().get_soldiers() as f32 * 2.4) as i32 {
                    (player, opponent, land_taken, dead_soldiers) = player.clone().attacked_by_neighbor(opponent);
                    players[counter] = opponent.clone().gain_land(land_taken);

                    attacked = true;
                }
            }

            if attacked != true {
                let mut evil_baron: Player = Player::new();
                evil_baron = evil_baron.init("Peppone".parse().unwrap(), true, "Monterana".parse().unwrap());
                (player, opponent, land_taken, dead_soldiers) = player.attacked_by_neighbor(evil_baron);
            }

            println!("\n\n{} {} of {} invades and seizes {} hectares of land!\n", opponent.clone().get_title(), opponent.clone().get_name(), opponent.clone().get_city(), land_taken);
            println!("{} {} loses {} soldiers in battle.\n", player.clone().get_title(), player.clone().get_name(), dead_soldiers);
        }
    }

    // Step 4: Generate income
    player = generate_income(player);

    // Step  5: Adjust taxes
    println!("\nEnter duty or tax to adjust");
    println!("1. Customs Duty, 2. Sales Tax, 3. Wealth Tax, 4. Justice");
    print!("Enter tax number for changes, q to continue: ");
    let answer: String = read!("{}\n");
    let answer_char = answer.chars().next().unwrap();

    while (answer_char != 'q') && (answer_char != 'Q') {
        if answer_char == '1' {
            print!("New customs duty (0 to 100): ");
            let answer: String = read!("{}\n");
            let mut duty = answer.parse::<f32>().unwrap();
            if duty > 100.0 {
                duty = 100.0;
            } else if duty < 0.0 {
                duty = 0.0;
            }
            player = player.clone().set_customs_duty(duty);
        } else if answer_char == '2' {
            print!("New sales tax (0 to 100): ");
            let answer: String = read!("{}\n");
            let mut duty = answer.parse::<f32>().unwrap();
            if duty > 50.0 {
                duty = 50.0
            } else if duty < 0.0 {
                duty = 0.0;
            }
            player = player.clone().set_sales_tax(duty);
        } else if answer_char == '3' {
            print!("New income tax (0 to 25): ");
            let answer: String = read!("{}\n");
            let mut duty = answer.parse::<f32>().unwrap();
            if duty > 25.0 {
                duty = 25.0
            } else if duty < 0.0 {
                duty = 0.0;
            }
            player = player.clone().set_income_tax(duty);
        } else if answer_char == '4' {
            print!("Justice: 1. Very fair, 2. Moderate, 3. Harsh, 4. Outrageous: ");
            let answer: String = read!("{}\n");
            let mut duty = answer.parse::<i32>().unwrap();
            if duty > 4 {
                duty = 4;
            }
            if duty < 1 {
                duty = 1;
            }
            player = player.clone().set_justice(duty);
        }
    }

    player = player.clone().adjust_tax();

        // Step 6: Purchases
    player = make_purchases(player.clone());

        // Step 7: Adding the revenue and check for the bankruptcy

    if player.clone().get_bankrupt() {
        println!("\n{} {} is bankrupt.", player.clone().get_title(), player.clone().get_name());
        println!("\nCreditors have seized much of your assets.");
        print!("(Press ENTER): ");

        let _: String = read!("{}\n");
    }

    if player.clone().get_year() == player.clone().get_year_of_death() {
        player = player.clone().set_dead();
    }

    let promoted: bool;
    (player, promoted) = player.check_new_title();

    if promoted {
        println!("\nCongratulations. Player {} has been promoted to {}.", player.clone().get_name(), player.clone().get_title());
    }

    if player.clone().get_title_num() >= 7 {
        player = player.clone().set_winner(true);
    }

    player = player.clone().set_next_year();

    return player;
}

fn make_purchases(mut player: Player) -> Player {
    let mut answer: String = ("s").to_string();
    let mut answer_char = answer.chars().next().unwrap();

    while (answer_char != 'q') && (answer_char != 'Q') {
        println!("\n\n{} {}\nState purchases.\n", player.clone().get_title(), player.clone().get_name());
        println!("1. Marketplace ({:.2})\t\t\t\t1000 florins\n", player.clone().get_market_places());
        println!("2. Woolen mill ({:.2})\t\t\t\t2000 florins\n", player.clone().get_mills());
        println!("3. Palace (partial) ({:.2})\t\t\t3000 florins\n", player.clone().get_palaces());
        println!("4. Cathedral (partial) ({:.2})\t\t\t5000 florins\n", player.clone().get_cathedral());
        println!("5. Equip one platoon of serfs as soldiers\t500 florins\n");
        println!("\nYou have {:.2} gold florins.\n", player.clone().get_treasury());
        println!("\nTo continue, enter q. To compare standings, enter 6\n");

        answer = read!("{}\n");
        answer_char = answer.chars().next().unwrap();

        if answer_char == '1' {
            player = player.buy_market();
        } else if answer_char == '2' {
            player = player.buy_mill();
        } else if answer_char == '3' {
            player = player.buy_palace();
        } else if answer_char == '4' {
            player = player.buy_cathedral();
        } else if answer_char == '5' {
            player = player.buy_soldiers();
        } else if answer_char == '6' {
//            show_stats(player.clone());
        }
    }

    return (player);
}

fn generate_income(mut player: Player) -> Player {
    let mut revenues :f32 = 0.0;
    let justice_level = vec!["---","Very Fair","Moderate","Harsh","Outrageous"];

    (player, revenues) = player.clone().generate_income();

    println!("State revenues {} gold florins.\n", revenues as i32);

    println!("\nCustoms Duty\tSales Tax\tIncome Tax\tJustice\n");
    println!("{:10.2}\t{:10.2}\t{:10.2}\t{:10.2} ({})\n",
          player.clone().get_customs_duty_revenue(),
          player.clone().get_sales_tax_revenue(),
          player.clone().get_income_tax_revenue(),
          player.clone().get_justice_revenue(),
          justice_level[player.clone().get_justice() as usize]);

    println!("({:10.2}%)\t({:10.2}%)\t({:10.2}%)",
          player.clone().get_customs_duty(),
          player.clone().get_sales_tax(),
          player.clone().get_income_tax());

    return player;
}

fn buy_and_sell_grain(mut player: Player) -> Player {
    let harvest_rating = vec![
        "Drought. Famine Threatens. ",
        "Bad Weather. Poor Harvest. ",
        "Normal Weather. Average Harvest. ",
        "Good Weather. Fine Harvest. ",
        "Excellent Weather. Great Harvest! "];

    let mut finished: bool = false;

    while finished == false {
        println!("\nYear {number:0>4} ", number=player.clone().get_year());
        println!("\n{} {}", player.clone().get_title(), player.clone().get_name());
        println!("\nRats ate {}% of your grain reserves ({} steres).", player.clone().get_rats(), player.clone().get_rats_ate());
        println!("\n{}\n", harvest_rating[player.clone().get_harvest()]);
        println!("\nGrain\t\tGrain\t\tPrice of\tPrice of\tTreasury");
        println!("Reserve\t\tDemand\t\tGrain\t\tLand\n");
        println!("{:.2}\t{:.2}\t{:.2}\t\t{:.2}\t\t{:.2}\n",player.clone().get_grain_reserve(),
                                                            player.clone().get_grain_demand(),
                                                            player.clone().get_grain_price(),
                                                            player.clone().get_land_price(),
                                                            player.clone().get_treasury());
        println!("steres\t\tsteres\t\t1000 st.\thectare\t\tgold florins\n");
        println!("\nYou have {} hectares of land.\n", player.clone().get_land());
        println!("\n1. Buy grain, 2. Sell grain, 3. Buy land, 4. Sell land ");

        print!("(Enter q to continue):");

        let mut answer: String = read!("{}\n");
        let answer_char = answer.chars().next().unwrap();

        if (answer_char == 'q') || (answer_char == 'Q') {
            finished = true;
        }

        if answer_char == '1' {
            print!("\nHow much grain do you want to buy (0 to specify a total)? ");
            let mut  answer: String = read!("{}\n");
            let mut amount = answer.parse::<f32>().unwrap();

            if amount == 0.0 {
                print!("\nHow much total grain do you wish? ");
                let mut  answer: String = read!("{}\n");
                amount = answer.parse::<f32>().unwrap();
                amount = amount - player.clone().get_grain_reserve();
            }

            if amount < 0.0 {
                print!("Invalid total amount.\n");
            } else {
                let ok :bool;
                (player, ok) = player.buy_grain(amount);

                if !ok {
                    println!("\nYou cannot effort it. Transaction not executed.");
                }
            }
        }

        if answer_char == '2' {
            print!("\nHow much grain do you want to sell (0 to specify a total)? ");
            let mut  answer: String = read!("{}\n");
            let mut amount = answer.parse::<f32>().unwrap();

            if amount == 0.0 {
                print!("\nHow much total grain do you wish? ");
                let mut  answer: String = read!("{}\n");
                amount = answer.parse::<f32>().unwrap();
                amount = amount - player.clone().get_grain_reserve();
            }

            if amount < 0.0 {
                print!("Invalid total amount.\n");
            } else {
                let ok :bool;
                (player, ok) = player.sell_grain(amount);

                if !ok {
                    println!("\nYou don't have it. Transaction not executed.");
                }
            }
        }

        if answer_char == '3' {
            print!("\nHow much land do you want to buy (0 to specify a total)? ");
            let mut answer: String = read!("{}\n");
            let mut amount = answer.parse::<i32>().unwrap();

            if amount == 0 {
                print!("\nHow much total land do you wish? ");
                let mut  answer: String = read!("{}\n");
                amount = answer.parse::<i32>().unwrap();
                amount = amount - player.clone().get_land();
            }

            if amount < 0 {
                print!("Invalid total amount.\n");
            } else {
                let ok :bool;
                (player, ok) = player.buy_land(amount);

                if !ok {
                    println!("\nYou cannot effort it. Transaction not executed.");
                }
            }
        }

        if answer_char == '4' {
            print!("\nHow much land do you want to sell (0 to specify a total)? ");
            let mut answer: String = read!("{}\n");
            let mut amount = answer.parse::<i32>().unwrap();

            if amount == 0 {
                print!("\nHow much total land do you wish? ");
                let mut  answer: String = read!("{}\n");
                amount = answer.parse::<i32>().unwrap();
                amount = player.clone().get_land() - amount;
            }

            if amount < 0 {
                print!("Invalid total amount.\n");
            } else {
                let ok :bool;
                (player, ok) = player.sell_land(amount);

                if !ok {
                    println!("\nYou don't have it. Transaction not executed.");
                }
            }
        }
    }

    return player;
}

fn release_grain(mut player: Player) -> Player {
    println!("Found: {} {} of {}", player.clone().get_title(), player.clone().get_name(), player.clone().get_city());

    let mut ok :bool = false;
    let mut amount: f32 = 0.0;

    while ok == false {
        println!("How much grain will you release for consumption?");
        print!("1 = Minimum ({:.2}), 2 = Maximum({:.2}), or enter a value: ",player.clone().get_minimum_grain(), player.clone().get_maximum_grain());
        let mut answer: String = read!("{}\n");

        amount = answer.parse::<f32>().unwrap();

        let mut too_little :bool = true;
        let mut too_much: bool = true;

        (too_little, too_much) = player.clone().release_grain_check(amount);

        // Are we being a Scrooge?
        if too_little {
            println!("You must release at least 20 % of your reserves.");
        } else if too_much {
        // Whoa. Slow down there son.
            println!("You must keep at least 20%.");
        } else {
            ok = true;
        }
    }

    player = player.process_released_grain(amount);

    // now let's check the results of our actions
    println!("You have a total of {} serfs in the city", player.clone().get_serfs());
    println!("\t{} serfs were born this year", player.clone().get_born_serfs());
    println!("\t{} serfs died this year", player.clone().get_dead_serfs());

    if (player.clone().get_immigrated_serfs() > 0) {
        println!("\t{} serfs immigrated into your city", player.clone().get_immigrated_serfs());
    }

    if (player.clone().get_fleeing_serfs() > 0) {
        println!("\t{} serfs flee harsh justice", player.clone().get_immigrated_serfs());
    }

    if (player.clone().get_market_revenue() > 0) {
        println!("\nYour markets made a win of {} florint", player.clone().get_market_revenue());
    }

    if (player.clone().get_mill_revenue() > 0) {
        println!("Your mills made a win of {} florint", player.clone().get_mill_revenue());
    }

    if (player.clone().get_soldier_pay() > 0) {
        println!("You paid your soldiers {} florint", player.clone().get_soldier_pay());
    }

    println!("(Press ENTER to continue)");
    let _ :String = read!("{}\n");

    return player;
}

fn rules() {
    println!("Santa Paravia and Fiumaccio\n");
    println!("You are the ruler of a 15th century Italian city state.\n");
    println!("If you rule well, you will receive higher titles. The first player to become king or queen wins.\n");
    println!("Life expectancy then was brief, so you may not live long enough to win.\n");
    println!("The computer provide you with information about your state:\n");
    println!("You gain wealth if you have enough land to give all your serfs space to raise crops. If you give them");
    println!("enough crops they will grow in numbers and produce more grain. If you distribute less");
    println!("grain, some of your people will starve, and you will have a high death rate. High taxes raise money, but slow down");
    println!("economic growth.  The markets make you money, they attract merchants. Churches get you clergy and raise your reputation");
    println!("But be aware: Your wealth might attract greedy neighbors to attack you...\n\n");
    println!("(Press ENTER to begin game)");

    let _ :String = read!("{}\n");
}
