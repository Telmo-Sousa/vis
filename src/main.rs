use plotters::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::time::{Duration, Instant};

mod algorithms;
use algorithms::*;

const ARRAY_SIZE: usize = 10000;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut data: Vec<i32> = (0..ARRAY_SIZE as i32).collect();
    data.reverse();

    let algorithms: Vec<(&str, fn(&mut [i32]))> = vec![
        ("Bubble Sort", bubble_sort),
        ("Cocktail Shaker Sort", cocktail_shaker_sort),
        ("Cycle Sort", cycle_sort),
        ("Gnome Sort", gnome_sort),
        ("Heap Sort", heap_sort),
        ("Insertion Sort", insertion_sort),
        ("Merge Sort", merge_sort),
        ("Quicksort", quicksort),
        ("Selection Sort", selection_sort),
        ("Shellsort", shell_sort),
    ];

    // Benchmark each algorithm
    let mut results: Vec<(String, Duration)> = Vec::new();
    for (name, algorithm) in algorithms {
        let mut test_data = data.clone();
        let start = Instant::now();
        algorithm(&mut test_data);
        let duration = start.elapsed();

        assert!(is_sorted(&test_data), "{} failed to sort correctly", name);

        results.push((name.to_string(), duration));
        println!("{}: {:?}", name, duration);
    }

    results.sort_by_key(|(_name, duration)| *duration);

    create_histogram(&results)?;

    Ok(())
}

fn is_sorted(data: &[i32]) -> bool {
    data.windows(2).all(|w| w[0] <= w[1])
}

fn create_histogram(results: &[(String, Duration)]) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("sorting_algorithms.png", (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    // Determine whether to use milliseconds or microseconds
    let use_ms = results.iter().any(|(_, duration)| duration.as_millis() > 100);

    let max_duration = if use_ms {
        results.last().map(|(_, d)| d.as_millis() as f64).unwrap_or(1.0)
    } else {
        results.last().map(|(_, d)| d.as_micros() as f64).unwrap_or(1.0)
    };

    let mut chart = ChartBuilder::on(&root)
        .caption(
            format!("Sorting Algorithm Performance (Array Size: {})", ARRAY_SIZE),
            ("sans-serif", 30).into_font(),
        )
        .margin(10)
        .x_label_area_size(60)
        .y_label_area_size(60)
        .build_cartesian_2d(
            0..results.len(),
            0.0..max_duration * 1.1,
        )?;

    chart.configure_mesh()
        .x_labels(results.len())
        .x_label_formatter(&|i| {
            if *i < results.len() {
                results[*i].0.clone()
            } else {
                String::new()
            }
        })
        .y_label_formatter(&|y| {
            if use_ms {
                format!("{:.2} ms", y)
            } else {
                format!("{:.2} μs", y)
            }
        })
        .y_desc(if use_ms { "Time (milliseconds)" } else { "Time (microseconds)" })
        .draw()?;

    chart.draw_series(
        results.iter().enumerate().map(|(i, (_, duration))| {
            let duration_value = if use_ms {
                duration.as_millis() as f64
            } else {
                duration.as_micros() as f64
            };

            let bar = Rectangle::new(
                [(i, 0.0), (i + 1, duration_value)],
                HSLColor(0.3 * (i as f64) / (results.len() as f64), 0.7, 0.5).filled(),
            );
            bar
        }),
    )?;

    chart.draw_series(
        results.iter().enumerate().map(|(i, (_, duration))| {
            let duration_value = if use_ms {
                duration.as_millis() as f64
            } else {
                duration.as_micros() as f64
            };

            let label = if use_ms {
                format!("{:.2} ms", duration_value)
            } else {
                format!("{:.2} μs", duration_value)
            };

            Text::new(
                label,
                (i, duration_value + max_duration * 0.02),
                ("sans-serif", 15).into_font(),
            )
        }),
    )?;

    root.present()?;
    println!("Chart has been saved to sorting_algorithms.png");

    Ok(())
}

