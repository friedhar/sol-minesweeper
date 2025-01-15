pub fn try_to_solve(grid: &[u8], width: usize, height: usize) -> Option<Vec<u8>> {
    if grid.len() != width * height {
        return None;
    }

    let mut solution = grid.to_vec();
    let mut changed = true;

    let get_adjacent_indices = |x: usize, y: usize| -> Vec<usize> {
        let mut indices = Vec::with_capacity(8);
        for dy in -1..=1i32 {
            for dx in -1..=1i32 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                    indices.push((ny as usize) * width + (nx as usize));
                }
            }
        }
        indices
    };

    while changed  {
        changed = false;
        let mut local_changes = 0;

        for y in 0..height {
            for x in 0..width {
                let idx = y * width + x;
                let cell = solution[idx];

                if cell >= 9 {
                    continue;
                }

                let adjacent = get_adjacent_indices(x, y);

                let mut mine_count = 0;
                let mut unknown_count = 0;
                for &adj_idx in &adjacent {
                    match solution[adj_idx] {
                        9 => unknown_count += 1,
                        10 => mine_count += 1,
                        _ => {}
                    }
                }

                if mine_count == cell {
                    for &adj_idx in &adjacent {
                        if solution[adj_idx] == 9 {
                            solution[adj_idx] = 0; // mark(safe)
                            changed = true;
                            local_changes += 1;
                        }
                    }
                }

                    if unknown_count > 0 && cell >= mine_count && cell - mine_count == unknown_count {

                    for &adj_idx in &adjacent {
                        if solution[adj_idx] == 9 {
                            solution[adj_idx] = 10; // mark(mine)
                            changed = true;
                            local_changes += 1;
                        }
                    }
                }
            }
        }

        if local_changes == 0 {
            break;
        }
    }

    for y in 0..height {
        for x in 0..width {
            let idx = y * width + x;
            let cell = solution[idx];

            if cell == 9 {
                return None; 
            }

            if cell < 9 {
                let adjacent = get_adjacent_indices(x, y);
                let mine_count = adjacent.iter()
                    .filter(|&&adj_idx| solution[adj_idx] == 10)
                    .count() as u8;

                if mine_count != cell {
                    return None; // invalid solution
                }
            }
        }
    }

    Some(solution)
}
