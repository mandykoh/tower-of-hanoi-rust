use core::fmt;

pub struct Disc {
    radius: usize,
}

pub struct Tower {
    discs: Vec<Disc>,
}

pub fn tower_with_discs(n: usize) -> Tower {
    let mut discs = vec![];
    for i in 0..n {
        discs.push(Disc { radius: n - i })
    }

    Tower { discs }
}

pub struct Puzzle {
    towers: Vec<Tower>,
    tower_height: usize,
    tower_radius: usize,
}

impl Puzzle {
    pub fn solve<F>(&mut self, each_move: F)
    where
        F: Fn(&Puzzle),
        F: Copy,
    {
        self.move_discs(
            self.towers[0].discs.len(),
            0,
            self.towers.len() - 1,
            each_move,
        );
    }

    fn move_discs<F>(&mut self, count: usize, from: usize, to: usize, each_move: F)
    where
        F: Fn(&Puzzle),
        F: Copy,
    {
        // Index of the free tower is not "from" and not "to"
        let free_tower = self.towers.len() - from - to;

        // Move all discs above us to the free tower
        if count > 1 {
            self.move_discs(count - 1, from, free_tower, each_move);
        }

        // Move us to the destination tower
        self.move_disc(from, to);
        each_move(self);

        // Move all discs from the free tower to above us
        if count > 1 {
            self.move_discs(count - 1, free_tower, to, each_move);
        }
    }

    fn move_disc(&mut self, from: usize, to: usize) {
        let disc = self.towers[from].discs.pop();
        self.towers[to].discs.push(disc.unwrap());
    }
}

impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for layer in 0..self.tower_height {
            for tower in &self.towers {
                let res = match tower.discs.get(self.tower_height - 1 - layer) {
                    Some(disc) => write!(
                        f,
                        "{}[{}|{}]{}",
                        " ".repeat(self.tower_radius - disc.radius),
                        "-".repeat(disc.radius),
                        "-".repeat(disc.radius),
                        " ".repeat(self.tower_radius - disc.radius)
                    ),

                    None => write!(
                        f,
                        "{}|{}",
                        " ".repeat(self.tower_radius + 1),
                        " ".repeat(self.tower_radius + 1)
                    ),
                };
                if res.is_err() {
                    return res;
                }
            }

            let res = writeln!(f);
            if res.is_err() {
                return res;
            }
        }

        writeln!(
            f,
            "{}",
            "=".repeat((self.tower_radius * 2 + 3) * self.towers.len())
        )
    }
}

pub fn puzzle_with_discs(count: usize) -> Puzzle {
    // Fully populate the first tower with discs
    let mut towers = vec![tower_with_discs(count)];

    // Add additional empty towers
    for _ in 0..2 {
        towers.push(tower_with_discs(0))
    }

    Puzzle {
        towers,
        tower_height: count + 1,
        tower_radius: count + 1,
    }
}
