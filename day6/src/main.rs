const MAX_REPRODUCTION: usize = 9;
const _TEST_CONTENTS: &str = "3,4,3,1,2";

struct LanternfishSchool {
    population: [usize; MAX_REPRODUCTION],
}

impl LanternfishSchool {
    fn new(contents: &str) -> LanternfishSchool {
        let mut population = [0 as usize; MAX_REPRODUCTION];
        contents.split(',').for_each(|fish| {
            let day = fish.parse::<usize>().unwrap();
            population[day] += 1;
        });

        LanternfishSchool { population }
    }

    fn next_day(&mut self) {
        let mut next_day_population = [0 as usize; MAX_REPRODUCTION];

        for (new_index, old_index) in (1..MAX_REPRODUCTION).enumerate() {
            next_day_population[new_index] = self.population[old_index];
        }

        next_day_population[6] += self.population[0];
        next_day_population[8] += self.population[0];

        self.population = next_day_population;
    }

    fn population_from_days(&mut self, days: usize) -> usize {
        for _ in 0..days {
            self.next_day();
        }

        self.population.iter().sum()
    }
}

fn main() {
    let contents = std::fs::read_to_string("day6/input.txt").unwrap();
    let mut school = LanternfishSchool::new(&contents);
    println!("Size of school: {}", school.population_from_days(256));
}
