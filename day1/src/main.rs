use std::fs;

/// Parse the contents string into a list of depths. Each line is a number.
// This function would need to be made more error resistant, etc. in a production environment.
fn parse(contents: &str) -> Vec<usize> {
    contents
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect()
}

/// Calculate the number of times the depth increases. I.E. The number of times value n in the slice
/// is less then n - 1.
///
/// * `depths` The slice of depths to determine the number of increases in.
fn increases_in_depth(depths: &[usize]) -> usize {
    depths
        .iter()
        .zip(&depths[1..])
        .filter(|(&a, &b)| a < b)
        .count()
}

/// Determine the number of increases in depths based for the given window size.
///
/// * `depths` The slice of depths to determine the number of increases in.
/// * `window` The size of the window to calculate before calculating the depths.
fn increases_in_depth_windows(depths: &[usize], window: usize) -> usize {
    let windowed_depths = depths
        .windows(window)
        .map(|w| w.iter().sum())
        .collect::<Vec<usize>>();
    increases_in_depth(&windowed_depths)
}

fn main() {
    // This contains the test values provided by the site.  Used for initial validation.
    const _TEST_VALUES: &[usize] = &[199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    let contents = fs::read_to_string("day1/input.txt").unwrap();
    let depths = parse(&contents);
    println!("Decreases in depths: {}", increases_in_depth(&depths));
    println!(
        "Decreases in windowed depths: {}",
        increases_in_depth_windows(&depths, 3)
    );
}
