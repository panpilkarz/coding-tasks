use std::fmt;
use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;
use permute::permute;

#[derive(Clone, Copy)]
struct Node {
    id: usize,
    x: f64,
    y: f64,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: ({:.2}, {:.2})", self.id, self.x, self.y)
    }
}

struct Route {
    path: Vec<usize>, // Node indices
    distance: f64,
}

impl fmt::Display for Route {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} distance={:.2}", self.path, self.distance)
    }
}

struct Tsp {
    matrix: Vec<Vec<f64>>,
    nodes: Vec<Node>,
}

impl Tsp {
    fn new(nodes: Vec<Node>) -> Self {
        // Calculate the matrix of distances between nodes
        let matrix = Tsp::calc_distance_matrix(&nodes);

        Tsp {nodes, matrix}
    }

    // Solve Traveling Salesman Problem using brute force
    fn brute_force(&self) -> Route {
        let mut best_path: (Vec<usize>, f64) = (Vec::new(), f64::MAX);

        for path in permute((0..self.nodes.len()).collect()) {
            let distance = self.calc_path_distance(&path);
            if distance < best_path.1 {
                best_path = (path, distance);
            }
        }

        Route{path: best_path.0, distance: best_path.1}
    }

    // Solve Traveling Salesman Problem using hill climbing
    fn hill_climb(&self) -> Route {
        if self.nodes.len() == 1 {
            return Route{path:vec![0], distance: 0.0}
        }

        // Hill climbing tries to find the best solution
        // by starting out with a random solution ...
        let path = random_path(self.nodes.len());
        let distance = self.calc_path_distance(&path);

        let mut current = Route{path, distance};

        // ... and then generate neighbours: solutions that
        // only slightly differ from the current one.
        // If the best of those neighbours is better than the current one,
        // it replaces the current solution with this better solution.
        // It then repeats the pattern by again creating neighbours.
        loop {
            // Gen all neighbours
            let neighbours = self.gen_neighbours(&current.path);

            // Get best neighbour
            let best_neighbour = self.get_best_neighbour(&neighbours);

            // Stop if best neighbour is worse than current solution
            if best_neighbour.distance >= current.distance {
                break
            }

            current = best_neighbour;
        }

        current
    }

    // Solve Traveling Salesman Problem using simulated annealing
    fn simulated_annealing(&self, initial_temp: f64, cooling_rate: f64, num_iterations: usize) -> Route {
        let mut temperature = initial_temp;
        let mut rng = rand::thread_rng();
        let n_nodes = self.nodes.len();

        // Start with random solution
        let mut path = random_path(self.nodes.len());

        for _ in 0..num_iterations {
            let current_distance = self.calc_path_distance(&path);

            let i = rng.gen_range(0..n_nodes);
            let j = rng.gen_range(0..n_nodes);
            path.swap(i, j);

            let new_distance = self.calc_path_distance(&path);
            let delta = new_distance - current_distance;

            // It is this acceptance probability, 
            // known as the Metropolis criterion,
            // that allows the algorithm to escape
            // from local minima when the temperature is high.
            if delta > 0.0 && f64::exp(-delta / temperature) < rng.gen::<f64>() {
                // Revert the swap if not accepted
                path.swap(i, j);
            }

            temperature *= cooling_rate;
        }
        let distance = self.calc_path_distance(&path);
        Route { path, distance }
    }

    // Generate neighbours by swapping two nodes in the given route
    fn gen_neighbours(&self, route: &Vec<usize>) -> Vec<Vec<usize>> {
        let mut neighbours: Vec<Vec<usize>> = Vec::new();

        for i in 0..route.len() {
            for j in i+1..route.len() {
                let mut neighbour = route.clone();
                neighbour[i] = route[j];
                neighbour[j] = route[i];
                neighbours.push(neighbour);
            }
        }

        neighbours
    }

    // Find the route with the shortest distance
    fn get_best_neighbour(&self, neighbours: &[Vec<usize>]) -> Route {
        let mut best_path_index = 0;
        let mut best_distance = self.calc_path_distance(&neighbours[0]);

        for (index, neighbour) in neighbours.iter().enumerate().skip(1) {
            let current_distance = self.calc_path_distance(neighbour);
            if current_distance < best_distance {
                best_path_index = index;
                best_distance = current_distance;
            }
        }
        Route {
            path: neighbours[best_path_index].clone(),
            distance: best_distance,
        }
    }

    // Calculate route total distance
    fn calc_path_distance(&self, route: &Vec<usize>) -> f64 {
        let mut total = 0.0;
        for i in 1..route.len() {
            total += self.matrix[route[i-1]][route[i]];
        }
        total += self.matrix[route[0]][route[route.len()-1]];
        total
    }

    // Calculate a matrix of distances between Nodes
    fn calc_distance_matrix(nodes: &Vec<Node>) -> Vec<Vec<f64>> {
        let mut distances: Vec<Vec<f64>> = Vec::new();
        for a in nodes {
            let mut row: Vec<f64> = Vec::new();
            for b in nodes {
                row.push(calc_distance(a, b));
            }
            distances.push(row);
        }
        distances
    }

    // A helper function
    #[allow(dead_code)]
    fn print_sample_routes(&self) {
        let n_samples = 10;
        let mut total_distance = 0.0;

        println!("Random routes:");
        for _ in 0..n_samples {
            let path = random_path(self.nodes.len());
            let distance = self.calc_path_distance(&path);
            let route = Route{
                path,
                distance,
            };
            total_distance += distance;
            println!("{}", route);
        }
        let avg_random_route_len = total_distance / n_samples as f64;
        println!("Average distance of random routes: {:.2}", avg_random_route_len);
    }
}

// Generate @n random nodes
fn random_nodes(n: usize) -> Vec<Node> {
    let mut rng = thread_rng();

    (0..).take(n).map(|num| Node{
        id: num,
        x: rng.gen_range(0.0..100.0),
        y: rng.gen_range(0.0..100.0),
    }).collect()

    // vec![
    //     Node{id: 0, x: 0.0, y: 0.0},
    //     Node{id: 10, x: 10.0, y: 0.0},
    //     Node{id: 20, x: 5.0, y: 5.0},
    //     Node{id: 30, x: 2.0, y: 3.0},
    // ]
}

// Generate random route
fn random_path(n: usize) -> Vec<usize> {
    let mut vec: Vec<usize> = (0..n).collect();
    vec.shuffle(&mut thread_rng());
    vec
}

// Calculate euclidean distance between two Nodes
fn calc_distance(a: &Node, b: &Node) -> f64 {
    f64::sqrt((a.x - b.x).powi(2) + (a.y - b.y).powi(2))
}

fn sep() {
    println!("{}", "-".repeat(60));
}

fn main() {
    // $ cargo run [number-of-nodes]
    // $ cargo run 10

    let n: usize = std::env::args().nth(1)
        .unwrap_or("8".to_string()).parse()
        .expect("Number of nodes expected");

    assert!(n > 0);

    // Random nodes
    let nodes = random_nodes(n);

    // Print nodes
    println!("Nodes:");
    for node in &nodes {
        println!("{}", node);
    }
    sep();

    // Init Tsp and print random routes
    let tsp = Tsp::new(nodes);
    tsp.print_sample_routes();
    sep();

    // 1/3 Run brute force (only if less than 10 nodes)
    if n < 10 {
        let bf_route = tsp.brute_force();
        println!("Best route (brute force):");
        println!("{}", bf_route);
        sep();
    }

    // 2/3 Run hill climbing
    let hc_route = tsp.hill_climb();
    println!("Best hill climbing route:");
    println!("{}", hc_route);
    sep();

    // 3/3 Run simulated annealing
    let initial_temp = 1000.0;
    let cooling_rate = 0.99;
    let num_iterations = 1000;

    let sa_route = tsp.simulated_annealing(initial_temp, cooling_rate, num_iterations);
    println!("Best simulated annealing route:");
    println!("{}", sa_route);
    sep();
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::{calc_distance, Node};

    #[test]
    fn test_calc_distance() {
        let a = Node{id: 0, x: 0.0, y: 3.0};
        let b = Node{id: 2, x: 4.0, y: 0.0};
        assert_eq!(calc_distance(&a, &b), 5.0);
    }

    // TODO use a fixed random seed to create deterministic tests
}
