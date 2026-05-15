use ::macroquad::prelude::*;
use ::rand::{rng, RngExt, rngs::ThreadRng};

const BORDER: f32 = 50.0;
const VEHICLE_CAPACITY: f32 = 100.0;
const DEPOT_TIME_WINDOW: f32 = 4000.0;

#[derive(Clone, Debug)]
struct Customer {
    pos: Vec2,
    demand: f32,
    ready: f32,
    due: f32,
    service: f32,
}

struct State {
    customers: Vec<Customer>,
    depot: Vec2,
    routes: Vec<Vec<usize>>, 
    rng: ThreadRng,
    temp: f64,
    total_dist: f32,
}

impl State {
    fn new(n: usize) -> Self {
        let mut rng = rng();
        let w = screen_width();
        let h = screen_height();
        let depot = vec2(w / 2.0, h / 2.0);

        let mut customers = Vec::new();
        for _ in 0..n {
            let pos = vec2(
                rng.random_range(BORDER..(w - BORDER)),
                rng.random_range(BORDER..(h - BORDER)),
            );
            let dist_from_depo = pos.distance(depot);
            customers.push(Customer {
                pos,
                demand: rng.random_range(10.0..30.0),
                ready: rng.random_range(0.0..500.0),
                due: rng.random_range((dist_from_depo + 100.0)..DEPOT_TIME_WINDOW),
                service: 20.0,
            });
        }

        let mut s = Self {
            customers,
            depot,
            routes: Vec::new(),
            rng,
            temp: 1000.0,
            total_dist: 0.0,
        };
        s.initial_solution();
        s
    }

    
    fn initial_solution(&mut self) {
        self.routes = (0..self.customers.len()).map(|i| vec![i]).collect();
        self.total_dist = self.calculate_all_dist();
    }

    fn soft_reset(&mut self) {
        self.initial_solution();
        self.temp = 1000.0;
    }

    fn calculate_route_dist(&self, route: &[usize]) -> f32 {
        if route.is_empty() { return 0.0; }
        let mut d = self.depot.distance(self.customers[route[0]].pos);
        for i in 0..route.len() - 1 {
            d += self.customers[route[i]].pos.distance(self.customers[route[i+1]].pos);
        }
        d += self.customers[*route.last().unwrap()].pos.distance(self.depot);
        d
    }

    fn calculate_all_dist(&self) -> f32 {
        self.routes.iter().map(|r| self.calculate_route_dist(r)).sum()
    }

    
    fn is_valid(&self, route: &[usize]) -> bool {
        let mut time = 0.0;
        let mut load = 0.0;
        let mut prev_pos = self.depot;

        for &idx in route {
            let c = &self.customers[idx];
            load += c.demand;
            if load > VEHICLE_CAPACITY { return false; }

            let travel = prev_pos.distance(c.pos);
            time = (time + travel).max(c.ready);
            
            if time > c.due { return false; }
            
            time += c.service;
            prev_pos = c.pos;
        }

        
        time + prev_pos.distance(self.depot) <= DEPOT_TIME_WINDOW
    }

    fn solve_exact(&mut self) {
        let mut best_routes = self.routes.clone();
        let mut best_dist = self.total_dist;
        
        let mut unassigned: Vec<usize> = (0..self.customers.len()).collect();
        let mut current_routes: Vec<Vec<usize>> = Vec::new();
        let mut iters = 0;
        
        self.bb_recursive(&mut unassigned, &mut current_routes, 0.0, &mut best_routes, &mut best_dist, &mut iters);
        
        if best_dist < self.total_dist {
            self.routes = best_routes;
            self.total_dist = best_dist;
        }
    }

    fn bb_recursive(
        &self,
        unassigned: &mut Vec<usize>,
        current_routes: &mut Vec<Vec<usize>>,
        current_dist: f32,
        best_routes: &mut Vec<Vec<usize>>,
        best_dist: &mut f32,
        iters: &mut usize
    ) {
        *iters += 1;
        // Limit iterations so the main thread doesn't lock up indefinitely for N=50
        if *iters > 5_000_000 {
            return;
        }

        if current_dist >= *best_dist {
            return;
        }

        if unassigned.is_empty() {
            *best_dist = current_dist;
            *best_routes = current_routes.clone();
            return;
        }

        let cust = unassigned.pop().unwrap();

        for r_idx in 0..current_routes.len() {
            for insert_pos in 0..=current_routes[r_idx].len() {
                let old_route_dist = self.calculate_route_dist(&current_routes[r_idx]);
                current_routes[r_idx].insert(insert_pos, cust);
                if self.is_valid(&current_routes[r_idx]) {
                    let new_route_dist = self.calculate_route_dist(&current_routes[r_idx]);
                    let new_dist = current_dist - old_route_dist + new_route_dist;
                    if new_dist < *best_dist {
                        self.bb_recursive(unassigned, current_routes, new_dist, best_routes, best_dist, iters);
                    }
                }
                current_routes[r_idx].remove(insert_pos);
            }
        }

        current_routes.push(vec![cust]);
        let new_route_dist = self.calculate_route_dist(&current_routes.last().unwrap());
        let new_dist = current_dist + new_route_dist;
        if new_dist < *best_dist {
            self.bb_recursive(unassigned, current_routes, new_dist, best_routes, best_dist, iters);
        }
        current_routes.pop();

        unassigned.push(cust);
    }

    fn update_sa(&mut self) {
        if self.temp < 0.01 { return; }

        let r1_idx = self.rng.random_range(0..self.routes.len());
        
        if self.rng.random_bool(0.5) && self.routes[r1_idx].len() >= 2 {
            
            let mut new_route = self.routes[r1_idx].clone();
            let i = self.rng.random_range(0..new_route.len());
            let j = self.rng.random_range(0..new_route.len());
            new_route.swap(i, j);

            if self.is_valid(&new_route) {
                let old_d = self.calculate_route_dist(&self.routes[r1_idx]);
                let new_d = self.calculate_route_dist(&new_route);
                let delta = (new_d - old_d) as f64;

                if delta < 0.0 || self.rng.random_range(0.0..1.0) < (-delta / self.temp).exp() {
                    self.routes[r1_idx] = new_route;
                    self.total_dist = self.calculate_all_dist();
                }
            }
        } else {
            
            let r2_idx = self.rng.random_range(0..self.routes.len());
            if r1_idx == r2_idx || self.routes[r1_idx].is_empty() { return; }

            let cust_idx = self.rng.random_range(0..self.routes[r1_idx].len());
            let customer = self.routes[r1_idx][cust_idx];

            let mut new_r1 = self.routes[r1_idx].clone();
            new_r1.remove(cust_idx);

            let mut new_r2 = self.routes[r2_idx].clone();
            let insert_pos = self.rng.random_range(0..=new_r2.len());
            new_r2.insert(insert_pos, customer);

            if self.is_valid(&new_r1) && self.is_valid(&new_r2) {
                let old_d = self.calculate_route_dist(&self.routes[r1_idx]) + self.calculate_route_dist(&self.routes[r2_idx]);
                let new_d = self.calculate_route_dist(&new_r1) + self.calculate_route_dist(&new_r2);
                let delta = (new_d - old_d) as f64;

                if delta < 0.0 || self.rng.random_range(0.0..1.0) < (-delta / self.temp).exp() {
                    self.routes[r1_idx] = new_r1;
                    self.routes[r2_idx] = new_r2;
                    self.routes.retain(|r| !r.is_empty());
                    self.total_dist = self.calculate_all_dist();
                }
            }
        }
        self.temp *= 0.99999;
    }

    fn draw(&self) {
        let colors = [
            Color::from_rgba(255, 0, 0, 255),
            Color::from_rgba(0, 255, 0, 255),
            Color::from_rgba(0, 0, 255, 255),
            Color::from_rgba(0, 255, 255, 255),
            Color::from_rgba(255, 0, 255, 255),
            Color::from_rgba(255, 255, 0, 255),
            Color::from_rgba(255, 255, 255, 255),
            Color::from_rgba(127, 0, 0, 255),
            Color::from_rgba(0, 127, 0, 255),
            Color::from_rgba(0, 0, 127, 255),
            Color::from_rgba(0, 127, 127, 255),
            Color::from_rgba(127, 0, 127, 255),
            Color::from_rgba(127, 127, 0, 255),
            Color::from_rgba(127, 127, 127, 255),
        ];
        //let colors = [RED, BLUE, GREEN, YELLOW, PURPLE, ORANGE, MAGENTA];
        for (i, route) in self.routes.iter().enumerate() {
            let color = colors[i % colors.len()];
            let mut prev = self.depot;
            for &idx in route {
                let curr = self.customers[idx].pos;
                draw_line(prev.x, prev.y, curr.x, curr.y, 2.0, color);
                prev = curr;
            }
            draw_line(prev.x, prev.y, self.depot.x, self.depot.y, 2.0, color);
        }

        draw_rectangle(self.depot.x - 10.0, self.depot.y - 10.0, 20.0, 20.0, WHITE);

        for c in &self.customers {
            draw_circle(c.pos.x, c.pos.y, 5.0, GRAY);
            draw_line(c.pos.x - 10.0, c.pos.y + 8.0, c.pos.x + 10.0, c.pos.y + 8.0, 1.0, WHITE);
        }

        draw_text(&format!("Vehicles: {}", self.routes.len()), 20.0, 30.0, 30.0, WHITE);
        draw_text(&format!("Distance: {:.2}", self.total_dist), 20.0, 60.0, 30.0, WHITE);
        draw_text(&format!("Temp: {:.2}", self.temp), 20.0, 90.0, 30.0, WHITE);
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "CVRPTW Solver".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let default_num: usize = 15;
    for _i in 0..60 {
        next_frame().await
    }
    let mut state = State::new(default_num);
    let mut working = false;
    let mut working_rendered = false;

    loop {
        clear_background(BLACK);

        if working && working_rendered {
            state.solve_exact();
            working = false;
            working_rendered = false;
        } else if working {
            working_rendered = true;
        }

        if !working {
            if is_key_down(KeyCode::Space) {
                for _ in 0..500 {
                    state.update_sa();
                }
            }

            if is_key_pressed(KeyCode::S) {
                state.soft_reset();
            }

            if is_key_pressed(KeyCode::R) {
                state = State::new(default_num);
            }

            if is_key_pressed(KeyCode::Escape) {
                break;
            }

            if is_key_pressed(KeyCode::B) {
                working = true;
                working_rendered = false;
            }
        }

        state.draw();

        if working {
            let text = "Working... (Branch and Bound)";
            let text_size = measure_text(text, None, 50, 1.0);
            draw_rectangle(
                screen_width() / 2.0 - text_size.width / 2.0 - 10.0,
                screen_height() / 2.0 - text_size.height / 2.0 - 10.0,
                text_size.width + 20.0,
                text_size.height + 20.0,
                Color::from_rgba(0, 0, 0, 200)
            );
            draw_text(text, screen_width() / 2.0 - text_size.width / 2.0, screen_height() / 2.0 + text_size.height / 2.0 - 10.0, 50.0, RED);
        }

        draw_text("Hold SPACE to Optimize | Press R to Reset | Press S to Soft Reset | Press B for Exact (B&B) | Press Esc to Exit", 20.0, screen_height() - 20.0, 20.0, LIGHTGRAY);
        
        next_frame().await
    }
}