pub mod example_countries;

use rand::Rng;
use std::fmt::Debug;
use itertools::Itertools;

pub struct AncientWorld {
    pub cities_count: usize,
    pub cities_names: Vec<String>,
    pub cities_roads: Vec<Vec<usize>>,
}

impl  Debug for AncientWorld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err = write!(
            f,
            "AncientWorld {{\n\tcities_count: {}",
            self.cities_count,
        );
        if let Err(_) = err { return  err; }

        let err = write!(f, "\n\tcities_names: [\n");
        if let Err(_) = err { return  err; }

        for (i, name) in self.cities_names.iter().enumerate() {
            let err = write!(f, "\t\tCity {}: {:5?},\n", i + 1, name);
            if let Err(_) = err { return  err; }
        }

        let err = write!(f, "\t]");
        if let Err(_) = err { return  err; }

        let err = write!(f, "\n\tcities_roads: [");
        if let Err(_) = err { return  err; }

        let err = write!(f, "\n\t\tCity №: {:5?}\n", (1..=self.cities_count).collect::<Vec<usize>>());
        if let Err(_) = err { return  err; }

        for (i, roads) in self.cities_roads.iter().enumerate() {
            let err = write!(f, "\t\t{:6}: {:5?},\n", i + 1, roads);
            if let Err(_) = err { return  err; }
        }

        let err = write!(f, "\t]");
        if let Err(_) = err { return  err; }

        write!(f, "\n}}")
    }
}

impl AncientWorld {
    pub fn new(cities_count: usize, cities_names: Vec<String>, cities_roads: Vec<Vec<usize>>) -> Option<Self> {
        if cities_count == 0 || cities_count != cities_names.len() || cities_count != cities_roads.len() {
            return None;
        }

        for roads in &cities_roads {
            if roads.len()!= cities_count {
                return None;
            }
        }

        return Some(AncientWorld {
            cities_count,
            cities_names,
            cities_roads,
        });
    }

    const MAX_CITY_NAME_LENGTH: usize = 15;

    fn generate_random_city_name(length: usize) -> String {
        const ALPHABET: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
        let mut rng = rand::thread_rng();
        let mut name = String::new();
        for _ in 0..length {
            let letter_index = rng.gen_range(0..ALPHABET.len());
            name.push(ALPHABET[letter_index] as char);
        }
        name
    }

    pub fn gen_rand_ancient_world(size: usize, road_range: usize) -> Self {
        let mut names = vec!["".to_string(); size];
        let mut roads = vec![vec![0; size]; size];
        
        let mut rng = rand::thread_rng();
        for i in 0..size {
            names[i] = AncientWorld::generate_random_city_name(rng.gen_range(1..=AncientWorld::MAX_CITY_NAME_LENGTH));
            for j in (i + 1)..size {
                let weight = rng.gen_range(1..=road_range);
                roads[i][j] = weight;
                roads[j][i] = weight;
            }
        }
        AncientWorld::new(size, names, roads).unwrap()
    }

    pub fn solve_tsp_by_brute_force(&self) -> (usize, Vec<String>) {
        if self.cities_count <= 1 {
            return (0, self.cities_names.clone());
        }

        fn calculate_distance(path: &[usize], roads: &Vec<Vec<usize>>) -> usize {
            path.windows(2).map(|w| roads[w[0]][w[1]]).sum::<usize>() + roads[*path.last().unwrap()][path[0]]
        }

        let mut shortest_path = Vec::new();
        let mut shortest_length = usize::MAX;

        let cities: Vec<usize> = (0..self.cities_count).collect();

        for perm in cities.iter().permutations(self.cities_count) {
            let path: Vec<usize> = perm.into_iter().map(|&idx| idx).collect();
            let distance = calculate_distance(&path, &self.cities_roads);
            if distance < shortest_length {
                shortest_length = distance;
                shortest_path = path;
            }
        }

        let path_names = shortest_path.iter().map(|&idx| self.cities_names[idx].clone()).collect();
        (shortest_length, path_names)
    }

    pub fn solve_tsp_by_ant_colony(
        &self,
        alpha: f64,
        beta: f64,
        evaporation: f64,
        ants: usize,
        days: usize,
        elite_ants: usize,
    ) -> (usize, Vec<String>) {
        if self.cities_count <= 1 {
            return (0, self.cities_names.clone());
        }

        let mut pheromones = vec![vec![1.0; self.cities_count]; self.cities_count];
        let mut best_path = Vec::new();
        let mut best_length = usize::MAX;
    
        for _ in 0..days {
            let mut all_paths = Vec::new();
            let mut all_lengths = Vec::new();
    
            for _ in 0..ants {
                let (path, length) = self.simulate_ant_path(&pheromones, alpha, beta);
                all_paths.push(path.clone());
                all_lengths.push(length);
    
                if length < best_length {
                    best_length = length;
                    best_path = path;
                }
            }
    
            self.update_pheromones(
                &mut pheromones,
                evaporation,
                &all_paths,
                &all_lengths,
                &best_path,
                best_length,
                elite_ants,
            );
        }
    
        let path_names = best_path.iter().map(|&idx| self.cities_names[idx].clone()).collect();
        (best_length, path_names)
    }
    
    fn update_pheromones(
        &self,
        pheromones: &mut Vec<Vec<f64>>,
        evaporation: f64,
        all_paths: &Vec<Vec<usize>>,
        all_lengths: &Vec<usize>,
        best_path: &Vec<usize>,
        best_length: usize,
        elite_ants: usize,
    ) {
        for i in 0..self.cities_count {
            for j in 0..self.cities_count {
                pheromones[i][j] *= 1.0 - evaporation;
            }
        }
    
        for (path, length) in all_paths.iter().zip(all_lengths.iter()) {
            for edge in path.windows(2) {
                let i = edge[0];
                let j = edge[1];
                pheromones[i][j] += 1.0 / *length as f64;
            }
            let i = path[path.len() - 1];
            let j = path[0];
            pheromones[i][j] += 1.0 / *length as f64;
        }
    
        for _ in 0..elite_ants {
            for edge in best_path.windows(2) {
                let i = edge[0];
                let j = edge[1];
                pheromones[i][j] += 1.0 / best_length as f64;
            }
            let i = best_path[best_path.len() - 1];
            let j = best_path[0];
            pheromones[i][j] += 1.0 / best_length as f64;
        }
    }    

    fn simulate_ant_path(
        &self,
        pheromones: &Vec<Vec<f64>>,
        alpha: f64,
        beta: f64,
    ) -> (Vec<usize>, usize) {
        let mut visited = std::collections::HashSet::new();
        let mut path = Vec::new();
        let mut current_city = 0;
        visited.insert(current_city);
        path.push(current_city);
        let mut length = 0;

        while visited.len() < self.cities_count {
            let chosen_city = self.choose_next_city(pheromones, &visited, current_city, alpha, beta);
            visited.insert(chosen_city);
            path.push(chosen_city);
            length += self.cities_roads[current_city][chosen_city];
            current_city = chosen_city;
        }

        length += self.cities_roads[current_city][path[0]];
        (path, length)
    }

    fn choose_next_city(
        &self,
        pheromones: &Vec<Vec<f64>>,
        visited: &std::collections::HashSet<usize>,
        current_city: usize,
        alpha: f64,
        beta: f64,
    ) -> usize {
        let mut probabilities = Vec::new();
        let mut total_prob = 0.0;

        for next_city in 0..self.cities_count {
            if !visited.contains(&next_city) {
                let pheromone = pheromones[current_city][next_city];
                let distance = self.cities_roads[current_city][next_city];
                let prob = pheromone.powf(alpha) * (1.0 / distance as f64).powf(beta);
                probabilities.push((next_city, prob));
                total_prob += prob;
            }
        }

        let mut choice = rand::random::<f64>() * total_prob;
        for (next_city, prob) in probabilities {
            if choice < prob {
                return next_city;
            }
            choice -= prob;
        }

        unreachable!("Не найдено города для транзита");
    }
}
