
use rand::Rng;
mod common;

#[derive(Clone, Debug)]

pub struct Player {
    cathedral: i32,
    clergy: i32,
    customs_duty: f32,
    customs_duty_revenue: f32,
    dead_serfs: i32,
    difficulty: i32,
    fleeing_serfs: i32,
    grain_demand :f32,
    grain_price :f32,
    grain_reserve :f32,
    harvest :i32,
    income_tax :f32,
    income_tax_revenue :f32,
    rats_ate :f32,
    justice :i32,
    justice_revenue :f32,
    land :i32,
    land_gained_from_attacks :i32,
    market_places :i32,
    market_revenue :i32,
    merchants :i32,
    mills_revenue :i32,
    mills :i32,
    new_serfs :i32,
    nobles :i32,
    old_title :usize,
    palace :i32,
    rats :i32,
    sales_tax :f32,
    sales_tax_revenue :f32,
    serfs :i32,
    soldiers_pay :i32,
    soldiers :i32,
    title_num :usize,
    transplanted_serfs :i32,
    treasury :f32,
    year :i32,
    year_of_death :i32,
    /* String variable parts */
    city :String,
    name :String,
    title :usize,
    /* Float variable parts */
    public_works: f32,
    land_price :i32,
    /* boolean variable parts */
    invade_me :bool,
    is_bankrupt :bool,
    is_dead :bool,
    i_won :bool,
    male :bool
}


impl Player {
    pub fn new() -> Player {
        let player: Player = Player {
            cathedral : 0,
            city : String::from(""),
            clergy : 5,
            customs_duty : 25.0,
            customs_duty_revenue : 0.0,
            dead_serfs : 0,
            difficulty : 1,
            fleeing_serfs : 0,
            grain_demand : 0.0,
            grain_price : 25.0,
            grain_reserve : 5000.0,
            harvest : 0,
            income_tax : 5.0,
            income_tax_revenue :  0.0,
            justice : 2,
            justice_revenue : 0.0,
            land : 10000,
            land_gained_from_attacks : 0,
            market_places : 0,
            market_revenue : 0,
            merchants : 25,
            mills_revenue : 0,
            mills : 0,
            new_serfs : 0,
            nobles : 4,
            old_title : 0,
            palace : 0,
            rats : 0,
            rats_ate : 0.0,
            sales_tax : 10.0,
            sales_tax_revenue : 0.0,
            serfs : 2000,
            soldiers_pay : 0,
            soldiers : 25,
            title_num : 0,
            transplanted_serfs : 0,
            treasury : 1000.0,
            year : 1400,
            year_of_death : 0,
            name : String::from(""),
            title : 0,
             /* Float variable parts */
            public_works : 0.0,
            land_price : 0,
             /* boolean variable parts */
            invade_me : false,
            is_bankrupt : false,
            is_dead : false,
            i_won : false,
            male : true
        };

        return player;
    }


    pub fn init(mut self, player_name: String, player_gender: bool, city_name: String, difficulty: i32) -> Player {
        self.city = city_name;
        self.name = player_name;
        self.male = player_gender;
        self.difficulty = difficulty;

        return self;
    }

    // String variable parts

    pub fn set_customs_duty(mut self, duty: f32) -> Player {
        self.customs_duty = duty;
        return self;
    }

    pub fn set_sales_tax(mut self, duty: f32) -> Player {
        self.sales_tax = duty;
        return self;
    }

    pub fn set_income_tax(mut self, duty: f32) -> Player {
        self.income_tax = duty;
        return self;
    }

    pub fn set_justice(mut self, duty: i32) -> Player {
        self.justice = duty;
        return self;
    }

    pub fn set_dead(mut self) -> Player {
        self.is_dead = true;
        return self;
    }

    pub fn set_next_year(mut self) -> Player {
        self.year = self.year + 1;
        return self;
    }

    pub fn set_winner(mut self, winner: bool) -> Player {
        self.i_won = winner;
        return self;
    }

    fn add_revenue(mut self) -> Player {
        self.treasury = self.treasury + self.justice_revenue + self.customs_duty_revenue;
        self.treasury = self.treasury + self.income_tax_revenue + self.sales_tax_revenue;

        if self.treasury < 0.0 {
           self.treasury *= 1.5;
        }

        if self.treasury < (-10000.0 * self.title as f32) {
           self.is_bankrupt = true;
        }

        return self;
    }

    pub fn adjust_tax(mut self) -> Player {
        self = self.add_revenue();

        if self.is_bankrupt == true {
            self = self.seize_assets();
        }

        return self;
    }


    pub fn attacked_by_neighbor(mut self, opponent:Player) -> (Player, Player, i32, i32) {
        let mut land_taken = (self.soldiers * 1000) - (self.land / 3);

        if land_taken > (self.land - 5000) {
            land_taken = (self.land - 5000) / 2
        }

        self.land -= land_taken;

        let mut dead_soldiers = rand::thread_rng().gen_range(0.. 40);

        if dead_soldiers > (self.soldiers - 15) {
            dead_soldiers = self.soldiers - 15;
        }

        self.soldiers -= dead_soldiers;

        return (self, opponent, land_taken, dead_soldiers);
    }

    pub fn buy_cathedral(mut self) -> Player {
        self.cathedral += 1;
        self.clergy += rand::thread_rng().gen_range(0.. 6);
        self.treasury = self.treasury - 5000.0;
        self.public_works += 1.0;

        return self;
    }

    pub fn buy_grain(mut self, amount:f32) -> (Player, bool) {
        let cost = amount * self.grain_price / 1000.0;
        if cost > self.treasury {
           return (self, false);
        }
        self.treasury = self.treasury - cost;
        self.grain_reserve += amount;

        return (self, true);
    }


    pub fn buy_land(mut self, amount:i32) -> (Player, bool) {
        let cost = amount * self.land_price;
        if cost > self.treasury as i32 {
            return (self, false);
        }

        self.land += amount;
        self.treasury = self.treasury - ((amount * self.land_price) as f32);

        return (self, true);
    }


    pub fn buy_market(mut self) -> Player {
        self.market_places += 1;
        self.merchants += 5;
        self.treasury = self.treasury - 1000.0;
        self.public_works += 1.0;

        return self;
    }


    pub fn buy_mill(mut self) -> Player {
        self.mills += 1;
        self.treasury = self.treasury - 2000.0;
        self.public_works += 0.25;

        return self;
    }


    pub fn buy_palace(mut self) -> Player {
        self.palace = self.palace + 1;
        self.nobles = self.nobles + rand::thread_rng().gen_range(0.. 2);
        self.treasury = self.treasury - 3000.0;
        self.public_works = self.public_works + 0.5;

        return self;
    }


    pub fn buy_soldiers(mut self) -> Player {
        self.soldiers = self.soldiers + 20;
        self.serfs = self.serfs - 20;
        self.treasury = self.treasury - 500.0;

        return self;
    }


    pub fn check_new_title(mut self) -> (Player, bool) {

        let mut total = common::limit10(self.market_places, 1);
        total += common::limit10(self.palace, 1);
        total += common::limit10(self.cathedral, 1);
        total += common::limit10(self.mills, 1);
        total += common::limit10(self.treasury as i32, 5000);
        total += common::limit10(self.land, 6000);
        total += common::limit10(self.merchants, 50);
        total += common::limit10(self.nobles, 5);
        total += common::limit10(self.soldiers, 50);
        total += common::limit10(self.clergy, 10);
        total += common::limit10(self.serfs, 2000);
        total += common::limit10(self.public_works as i32 * 100, 500);

        self.title = (total / self.difficulty - self.justice) as usize;

        if self.title > 7 {
            self.title = 7;
        }

        if self.title < 0 {
            self.title = 0;
        }

        if self.title > self.old_title {
            self.title = self.old_title + 1;
            self.old_title = self.title;

            if self.title >= 7 {
                self.i_won = true;
            }

            return (self, true);
        }

        self.title = self.old_title;
        return (self, false);
    }


    pub fn generate_income(mut self) -> (Player, f32) {
        self.justice_revenue = ((self.justice * 300 - 500) * self.title as i32) as f32;

        let mut revenue_base :f32 = 150.0 - (self.sales_tax - self.customs_duty - self.income_tax);

        if revenue_base < 1.0 {
            revenue_base = 1.0;
        }

        revenue_base = revenue_base / 100.0;

        self.customs_duty_revenue = (self.nobles * 180 + self.clergy * 75 + self.merchants * 20) as f32 * revenue_base ;
        self.customs_duty_revenue = self.customs_duty_revenue + (self.public_works) * 100.0;
        self.customs_duty_revenue = self.customs_duty / 100.0 * self.customs_duty_revenue;

        self.sales_tax_revenue = (self.nobles * 50 + self.merchants * 25) as f32 + self.public_works * 10.0;
        self.sales_tax_revenue = self.sales_tax_revenue * revenue_base * (5 - self.justice) as f32 * self.sales_tax;
        self.sales_tax_revenue = self.sales_tax_revenue / 200.0;

        self.income_tax_revenue = (self.nobles * 250) as f32 + self.public_works * 20.0;
        self.income_tax_revenue = self.income_tax_revenue + (10 * self.justice * self.nobles) as f32 * revenue_base;
        self.income_tax_revenue = self.income_tax_revenue * self.income_tax;
        self.income_tax_revenue = self.income_tax_revenue / 100.0;

        let revenues :f32 = self.customs_duty_revenue + self.sales_tax_revenue + self.income_tax_revenue + self.justice_revenue;

        return (self, revenues);
    }


    pub fn dead(self) -> bool {
        return self.is_dead;
    }


    /**
    Function to store land gained from attacks.
    */
    pub fn gain_land(mut self, gained_land :i32) -> Player {
        self.land_gained_from_attacks = self.land_gained_from_attacks + gained_land;
        return self;
    }

    pub fn harvest_land_and_grain_prices(mut self) -> Player {
        self.harvest = (rand::thread_rng().gen_range(0.. 5) + rand::thread_rng().gen_range(0.. 6)) / 2;

        if self.harvest > 5 {
            self.harvest = 5;
        } else if self.harvest < 1 {
            self.harvest = 1;
        }

        // Generate an offset for use in later int -> float conversions.
        // we are using 8-bit random numbers
        let my_random :f32 = (rand::thread_rng().gen_range(0.. 32767) as f32) / 32767.0;

        // If you think this Rust code is ugly, you should see the original BASIC.
        let mut worked_land :f32 = self.land as f32;

        // available work force = number of serfs - number of serfs needed for mills (1 serf per mill) times 100
        let available_work_force :f32 = ((self.serfs - self.mills) * 100) as f32;

        // every unit available work force can work on 5 ha of land
        let mut processable_land :f32 = available_work_force * 5.0;

        if processable_land < 0.0 {
            processable_land = 0.0;
        }

        if processable_land < worked_land {
            worked_land = processable_land;
        }

        // to grow grain, 2 units of grain are needed per ha of land
        let max_land_to_be_used_due_to_grain = self.grain_reserve * 2.0;

        if max_land_to_be_used_due_to_grain < worked_land {
            worked_land = max_land_to_be_used_due_to_grain;
        }

        let harvest_per_ha :f32 = self.harvest as f32 + my_random - 0.5;

        let mut harvest = harvest_per_ha * worked_land;

        self.grain_reserve = self.grain_reserve + harvest;

        // calculating grain demand
        self.grain_demand = ((self.nobles * 100) + (self.cathedral * 40) + (self.merchants * 30) + (self.soldiers * 10) + (self.serfs * 5)) as f32;

        // calculating land price
        self.land_price = (3 * (self.harvest) + (rand::thread_rng().gen_range(0.. 6)) + 10) / 10;

        if harvest < 0.0 {
            harvest = harvest * -1.0;
        }

        let mut grain_demand_coverage :f32;
        if harvest < 1.0 {
            grain_demand_coverage = 2.0
        } else {
            grain_demand_coverage = self.grain_demand / harvest;
        }

        if grain_demand_coverage > 2.0 {
            grain_demand_coverage = 2.0;
        }

        if grain_demand_coverage < 0.8 {
            grain_demand_coverage = 0.8;
        }

        self.land_price = self.land_price * grain_demand_coverage as i32;

        if self.land_price < 1 {
            self.land_price = 1;
        }

        self.grain_price = 6.0 - (self.harvest as f32) * 3.0 + (rand::thread_rng().gen_range(0.. 5) as f32) + (rand::thread_rng().gen_range(0.. 5) as f32) * 4.0 * grain_demand_coverage;

        if self.grain_price < 0.0 {
            self.grain_price = 0.1;
        }

        return self;
    }


    pub fn process_released_grain(mut self, mut released_grain :f32) -> Player {
        self.soldiers_pay = 0;
        self.market_revenue = 0;
        self.new_serfs = 0;
        self.dead_serfs = 0;
        self.transplanted_serfs = 0;
        self.fleeing_serfs = 0;

        self.invade_me = false;

        if released_grain == 1.0 {
            released_grain = self.clone().get_minimum_grain();
        }

        if released_grain == 2.0 {
            released_grain = self.clone().get_maximum_grain();
        }

        self.grain_reserve = self.grain_reserve - released_grain;

        let mut demand_satisfaction = released_grain / (self.grain_demand - 1.0);

        if demand_satisfaction > 0.0 {
            demand_satisfaction = demand_satisfaction / 2.0;
        }

        if demand_satisfaction > 0.25 {
            demand_satisfaction = demand_satisfaction / 10.0 + 0.25;
        }

        // calculate current taxation level as 50% - all taxes
        let mut happiness_factor :f32 = 50.0 - self.customs_duty - self.sales_tax - self.income_tax;

        // if all taxes exceed the 50%, multiply the number by the justice value
        if happiness_factor < 0.0 {
            happiness_factor = happiness_factor * self.justice as f32;
            // the harsher the justice, the more unhappy the people are
        }

        happiness_factor = happiness_factor / 10.0;

        // if the people are positive about their situation
        if happiness_factor > 0.0 {
            //decrement the value due to the justice
            happiness_factor = happiness_factor + 3.0 - self.justice as f32;
        }

        demand_satisfaction = demand_satisfaction + (happiness_factor / 10.0);

        if demand_satisfaction > 0.5 {
            // if the demand satisfaction exceeds .5, the cap it.
            demand_satisfaction = 0.5
        }

        let grain_demand = self.grain_demand;
        let mut grain_deficit;
        if released_grain < (grain_demand - 1.0) {
            grain_deficit = (grain_demand - released_grain) / (grain_demand * 100.0 - 9.0);

            let mut unhappiness_factor = grain_deficit;

            if grain_deficit > 65.0 {
                grain_deficit = 65.0
            }

            if grain_deficit < 0.0 {
                unhappiness_factor = 0.0;
            }

            self = self.serfs_procreating(3.0);
            self = self.serfs_decomposing(unhappiness_factor + 8.0);
        } else {
            self = self.serfs_procreating(7.0 * demand_satisfaction);
            self = self.serfs_decomposing(3.0);

            if (self.customs_duty + self.sales_tax) < 35.0 {
                // if customs duty and sales tax combined are less than 35%
                self.merchants += rand::thread_rng().gen_range(0.. 4);
            }

            if self.income_tax < rand::thread_rng().gen_range(0.. 28) as f32 {
                self.nobles += rand::thread_rng().gen_range(0.. 2);
                self.clergy += rand::thread_rng().gen_range(0.. 3);
            }

            // overachieving annual results: 30% extra released
            if released_grain > self.grain_demand * 1.3 {
                let population_density = (self.serfs as f32) / 1000.0;
                let transplanting_serfs :i32 = ((released_grain - (self.grain_demand)) /
                    (self.grain_demand * 10.0) *
                    population_density) as i32 *
                    (rand::thread_rng().gen_range(0.. 25)) + (rand::thread_rng().gen_range(0.. 40));
                self.transplanted_serfs = transplanting_serfs;
                self.serfs = self.serfs + self.transplanted_serfs;

                let mut immigration_pull = transplanting_serfs;
                immigration_pull = immigration_pull * (rand::thread_rng().gen_range(0.. 100) / 100);

                if immigration_pull > 50 {
                    immigration_pull = 50;
                }

                self.merchants = self.merchants + immigration_pull;
                self.nobles = self.nobles + 1;
                self.clergy = self.clergy + 2;
            }
        }

        if self.justice > 2 {
            self.justice_revenue = (self.serfs / 100 * (self.justice - 2) * (self.justice - 2)) as f32;
            self.justice_revenue = rand::thread_rng().gen_range(0..  (self.justice_revenue as i32)) as f32;
            self.serfs = self.serfs - self.justice_revenue as i32;
            self.fleeing_serfs = self.justice_revenue as i32;
        }

        self.market_revenue = self.market_places * 75;

        if self.market_revenue > 0 {
            self.treasury = self.treasury + (self.market_revenue as f32);
        }

        self.mills_revenue = self.mills * (55 + rand::thread_rng().gen_range(0..  250));

        if self.mills_revenue > 0 {
            self.treasury = self.treasury + (self.mills_revenue as f32);
        }

        self.soldiers_pay = self.soldiers * 3;
        self.treasury = self.treasury - (self.soldiers as f32);

        if (self.land / 1000) > self.soldiers {
            self.invade_me = false;
        }

        if (self.land / 500) > self.soldiers {
            self.invade_me = true;
        }

        return self;
    }

    pub fn rat_loss(mut self) -> Player {
        self.rats = rand::thread_rng().gen_range(0.. 50);
        self.rats_ate = self.grain_reserve * self.rats as f32 / 100.0;
        self.grain_reserve = self.grain_reserve - (self.rats_ate);

        return self;
    }

    pub fn release_grain_check(self, mut released_grain :f32) -> (bool, bool) {
        let mut too_little = false;
        let mut too_much :bool = false;

        if released_grain == 1.0 {
            released_grain = self.clone().get_minimum_grain();
        }

        if released_grain == 2.0 {
            released_grain = self.clone().get_maximum_grain();
        }

        if (released_grain + 1.0) < self.clone().get_minimum_grain() {
            // Are we being a Scrooge?
            too_little = true;
        } else if (released_grain - 1.0) > self.clone().get_maximum_grain() {
            too_much = true;
        }

        return(too_little, too_much);
    }


    // seizing assets from a bankrupt player.
    fn  seize_assets(mut self) -> Player {
        self.market_places = 0;
        self.palace = 0;
        self.cathedral = 0;
        self.mills = 0;
        self.land = 6000;
        self.public_works = 1.0;
        self.treasury = 100.0;
        self.is_bankrupt = true;

        return self;
    }


    pub fn sell_grain(mut self, amount :f32) -> (Player, bool) {
        if amount > self.grain_reserve {
            return (self, false);
        }

        self.treasury = self.treasury + (amount * self.grain_price / 1000.0);
        self.grain_reserve = self.grain_reserve - amount;

        return (self, true);
    }


    pub fn sell_land(mut self, amount: i32) -> (Player, bool) {
        if amount > (self.land - 5000) {
            return (self, false);
        }

        self.land = self.land - amount;
        self.treasury = self.treasury + (amount * self.land_price) as f32;

        return (self, true);
    }


    fn serfs_decomposing(mut self, decomposing_base :f32) -> Player {
        // split decomposingBase into the part before and after the decimal
        let decomposing_rate = decomposing_base as i32;
        let overkill = decomposing_base - decomposing_rate as f32;

        self.dead_serfs = (rand::thread_rng().gen_range(0..  decomposing_rate) + (overkill as i32)) * (self.serfs) / 100;
        self.serfs = self.serfs - self.dead_serfs;

        return self;
    }


    fn serfs_procreating(mut self, procreation_base :f32) -> Player {
        // split procreationBase into the part before and after the decimal
        let procreation_rate :i32 = procreation_base as i32;
        let birth_surplus = procreation_base - procreation_rate as f32;

        self.new_serfs = (rand::thread_rng().gen_range(0..  procreation_rate) + (birth_surplus as i32)) * (self.serfs) / 100;
        self.serfs = self.serfs + self.new_serfs;

        return self;
    }

    //
    // list of all getters for the class
    //
    pub fn get_cathedral(self) -> i32 {
        return self.cathedral;
    }

    pub fn get_maximum_grain(self) -> f32 {
        return self.grain_reserve - self.clone().get_minimum_grain();
    }

    pub fn get_minimum_grain(self) -> f32 {
        return self.grain_reserve / 5.0;
    }

    pub fn get_title(self) -> String {
        let male_titles: [&str; 8] = ["Sir", "Baron", "Count", "Marquis", "Duke", "Grand Duke", "Prince", "* H.R.H. King"];
        let female_titles: [&str; 8] = ["Lady", "Baroness", "Countess", "Marquise", "Duchess", "Grand Duchess", "Princess", "* H.R.H. Queen"];

        if self.title > 7 {
            panic!("No title greater 7 allowed");
        }

        let title :String;

        if self.male == true {
            title = male_titles[self.title].parse().unwrap();
        } else {
            title = female_titles[self.title].parse().unwrap();
        }

        return title;
    }

    /**
    Consolidate the gained land:
    */
    pub fn consolidate(mut self) -> Player {
        self.land = self.land + self.land_gained_from_attacks;
        self.land_gained_from_attacks = 0;
        return self;
    }

    pub fn get_title_num(self) -> i32 {
        return self.title_num as i32;
    }

    pub fn get_name(self) -> String {
        return self.name;
    }

    pub fn get_city(self) -> String {
        return self.city;
    }

    pub fn get_year(self) -> i32 {
        return self.year;
    }

    pub fn i_won(self) -> bool {
        return self.i_won;
    }

    pub fn get_nobles(self) -> i32 {
        return self.nobles;
    }

    pub fn get_clergy(self) -> i32 {
        return self.clergy;
    }

    pub fn get_merchants(self) -> i32 {
        return self.merchants;
    }

    pub fn get_rats(self) -> i32 {
        return self.rats;
    }

    pub fn get_rats_ate(self) -> f32 {
        return self.rats_ate;
    }

    pub fn get_harvest(self) -> usize {
        return (self.harvest - 1) as usize;
    }

    pub fn get_grain_reserve(self) -> f32 {
        return self.grain_reserve;
    }

    pub fn get_grain_demand(self) -> f32 {
        return self.grain_demand;
    }

    pub fn get_grain_price(self) -> f32 {
        return self.grain_price;
    }

    pub fn get_land(self) -> i32 {
        return self.land;
    }

    pub fn get_land_price(self) -> i32 {
        return self.land_price;
    }

    pub fn get_palaces(self) -> i32 {
        return self.palace;
    }

    pub fn get_treasury(self) -> f32 {
        return self.treasury;
    }

    pub fn get_serfs(self) -> i32 {
        return self.serfs;
    }

    pub fn get_born_serfs(self) -> i32 {
        return self.new_serfs;
    }

    pub fn get_dead_serfs(self) -> i32 {
        return self.dead_serfs;
    }

    pub fn get_immigrated_serfs(self) -> i32 {
        return self.transplanted_serfs;
    }

    pub fn get_fleeing_serfs(self) -> i32 {
        return self.fleeing_serfs;
    }

    pub fn get_market_places(self) -> i32 {
        return self.market_places;
    }

    pub fn get_market_revenue(self) -> i32 {
        return self.market_revenue;
    }

    pub fn get_mills(self) -> i32 {
        return self.mills;
    }

    pub fn get_mill_revenue(self) -> i32 {
        return self.mills_revenue;
    }

    pub fn get_soldiers(self) -> i32 {
        return self.soldiers;
    }

    pub fn get_soldier_pay(self) -> i32 {
        return self.soldiers_pay;
    }

    pub fn get_invade_me(self) -> bool {
        return self.invade_me;
    }

    pub fn get_customs_duty(self) -> f32 {
        return self.customs_duty;
    }

    pub fn get_customs_duty_revenue(self) -> f32 {
        return self.customs_duty_revenue;
    }

    pub fn get_sales_tax(self) -> f32 {
        return self.sales_tax;
    }

    pub fn get_sales_tax_revenue(self) -> f32 {
        return self.sales_tax_revenue;
    }

    pub fn get_income_tax(self) -> f32 {
        return self.income_tax;
    }

    pub fn get_income_tax_revenue(self) -> f32 {
        return self.income_tax_revenue;
    }

    pub fn get_justice_revenue(self) -> f32 {
        return self.justice_revenue;
    }

    pub fn get_justice(self) -> i32 {
        return self.justice;
    }

    pub fn get_bankrupt(self) -> bool {
        return self.is_bankrupt;
    }

    pub fn get_year_of_death(self) -> i32 {
        return self.year_of_death;
    }

    //
    // list of all setters for the class
    //

}