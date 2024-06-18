use plotters::prelude::*;
use std::time::{Duration, Instant};


fn bubble_sort(arr: &mut [i32]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..len - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
            
        }
    }
}

fn cocktail_sort(arr: &mut [i32]) {
    let mut left = 0;
    let mut right = arr.len() - 1;

    while left <= right {
        for i in left..right {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
            }
            
        }
        right -= 1;

        for i in (left..right).rev() {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
            }
            
        }
        left += 1;
    }
}

fn cycle_sort(arr: &mut [i32]) {
    let len = arr.len();
    for cycle_start in 0..len {
        let mut item = arr[cycle_start];
        let mut pos = cycle_start;
        for i in cycle_start + 1..len {
            if arr[i] < item {
                pos += 1;
            }
        }
        if pos == cycle_start {
            continue;
        }
        while item == arr[pos] {
            pos += 1;
        }
        if pos != cycle_start {
            std::mem::swap(&mut item, &mut arr[pos]);
        }
        
        while pos != cycle_start {
            pos = cycle_start;
            for i in cycle_start + 1..len {
                if arr[i] < item {
                    pos += 1;
                }
            }
            while item == arr[pos] {
                pos += 1;
            }
            if item != arr[pos] {
                std::mem::swap(&mut item, &mut arr[pos]);
            }
            
        }
    }
}

fn gnome_sort(arr: &mut [i32]) {
    let mut index = 0;
    while index < arr.len() {
        if index == 0 || arr[index] >= arr[index - 1] {
            index += 1;
        } else {
            arr.swap(index, index - 1);
            
            index -= 1;
        }
    }
}

fn heap_sort(arr: &mut [i32]) {
    fn heapify(arr: &mut [i32], n: usize, i: usize) {
        let mut largest = i;
        let left = 2 * i + 1;
        let right = 2 * i + 2;

        if left < n && arr[left] > arr[largest] {
            largest = left;
        }

        if right < n && arr[right] > arr[largest] {
            largest = right;
        }

        if largest != i {
            arr.swap(i, largest);
            heapify(arr, n, largest);
        }
    }

    let len = arr.len();

    for i in (0..len / 2).rev() {
        heapify(arr, len, i);
    }

    for i in (1..len).rev() {
        arr.swap(0, i);
        
        heapify(arr, i, 0);
    }
}

fn insertion_sort(arr: &mut [i32]) {
    let len = arr.len();
    for i in 1..len {
        let mut j = i;
        while j > 0 && arr[j] < arr[j - 1] {
            arr.swap(j, j - 1);
            
            j -= 1;
        }
    }
}

fn merge_sort(arr: &mut [i32]) {
    fn merge(arr: &mut [i32], left: usize, mid: usize, right: usize) {
        let mut temp: Vec<i32> = Vec::new();
        let mut i = left;
        let mut j = mid + 1;

        while i <= mid && j <= right {
            if arr[i] <= arr[j] {
                temp.push(arr[i]);
                i += 1;
            } else {
                temp.push(arr[j]);
                j += 1;
            }
        }

        while i <= mid {
            temp.push(arr[i]);
            i += 1;
        }

        while j <= right {
            temp.push(arr[j]);
            j += 1;
        }

        for i in left..=right {
            arr[i] = temp[i - left];
        }
    }

    fn merge_sort_recursive(arr: &mut [i32], left: usize, right: usize) {
        if left < right {
            let mid = left + (right - left) / 2;

            merge_sort_recursive(arr, left, mid);
            merge_sort_recursive(arr, mid + 1, right);

            merge(arr, left, mid, right);
            
        }
    }

    let len = arr.len();
    merge_sort_recursive(arr, 0, len - 1);
}

fn quicksort(arr: &mut [i32]) {
    fn partition(arr: &mut [i32], low: usize, high: usize) -> usize {
        let pivot = arr[high];
        let mut i = low;
        for j in low..high {
            if arr[j] < pivot {
                arr.swap(i, j);
                i += 1;
            }
        }
        arr.swap(i, high);
        i
    }

    fn quicksort_recursive(arr: &mut [i32], low: usize, high: usize) {
        if low < high {
            let pi = partition(arr, low, high);
            
            if pi > 0 {
                quicksort_recursive(arr, low, pi - 1);
            }
            quicksort_recursive(arr, pi + 1, high);
        }
    }

    let len = arr.len();
    quicksort_recursive(arr, 0, len - 1);
}

fn selection_sort(arr: &mut [i32]) {
    let len = arr.len();
    for i in 0..len {
        let mut min_idx = i;
        for j in i + 1..len {
            if arr[j] < arr[min_idx] {
                min_idx = j;
            }
        }
        arr.swap(i, min_idx);
        
    }
}

fn shellsort(arr: &mut [i32]) {
    let len = arr.len() as i32;
    let mut gap = len / 2;

    while gap > 0 {
        for i in gap..len {
            let temp = arr[i as usize];
            let mut j = i;

            while j >= gap && arr[(j - gap) as usize] > temp {
                arr[j as usize] = arr[(j - gap) as usize];
                j -= gap;
            }

            arr[j as usize] = temp;
        }
        

        gap /= 2;
    }
}

fn reset_and_sort<F>(sort_fn: F) -> Duration
where
    F: Fn(&mut [i32]),
{
    let mut array: Vec<i32> = (0..10000).rev().collect(); 

    // println!("---------------------------------");
    // println!("Original array:");
    // println!("---------------------------------");
    // print_array(&array);

    let start_time = Instant::now();
    sort_fn(&mut array);
    let duration = start_time.elapsed();

    // println!("Sorted array:");
    // print_array(&array);
    println!("Time taken: {:?}", duration);

    duration
}

fn main() {
    let mut results = Vec::new();

    println!("---------------------------------");
    println!("Sorting by Bubble Sort:");
    results.push(("Bubble Sort", reset_and_sort(bubble_sort)));
    println!("---------------------------------");

    println!("Sorting by Cocktail Shaker Sort:");
    results.push(("Cocktail Shaker Sort", reset_and_sort(cocktail_sort)));
    println!("---------------------------------");

    println!("Sorting by Cycle Sort:");
    results.push(("Cycle Sort", reset_and_sort(cycle_sort)));
    println!("---------------------------------");

    println!("Sorting by Gnome Sort:");
    results.push(("Gnome Sort", reset_and_sort(gnome_sort)));
    println!("---------------------------------");

    println!("Sorting by Heap Sort:");
    results.push(("Heap Sort", reset_and_sort(heap_sort)));
    println!("---------------------------------");

    println!("Sorting by Insertion Sort:");
    results.push(("Insertion Sort", reset_and_sort(insertion_sort)));
    println!("---------------------------------");

    println!("Sorting by Merge Sort:");
    results.push(("Merge Sort", reset_and_sort(merge_sort)));
    println!("---------------------------------");

    println!("Sorting by Quicksort:");
    results.push(("Quicksort", reset_and_sort(quicksort)));
    println!("---------------------------------");

    println!("Sorting by Selection Sort:");
    results.push(("Selection Sort", reset_and_sort(selection_sort)));
    println!("---------------------------------");

    println!("Sorting by Shellsort:");
    results.push(("Shellsort", reset_and_sort(shellsort)));
    println!("---------------------------------");

    let root_area = BitMapBackend::new("sort_performance.png", (1920, 1080)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let max_duration = results
        .iter()
        .map(|(_, duration)| duration.as_millis())
        .max()
        .unwrap_or(0) as i32;

    let mut chart = ChartBuilder::on(&root_area)
        .caption(
            "Sorting Algorithm Performance on a Reversed Array of Size 10000",
            ("sans-serif", 30).into_font(),
        )
        .margin(20)
        .x_label_area_size(50) 
        .y_label_area_size(50)
        .build_cartesian_2d((0..results.len() - 1).into_segmented(), 0..max_duration)
        
        .unwrap();

    chart
        .configure_mesh()
        .disable_x_mesh() 
        .x_labels(results.len())
        .x_label_formatter(&|x| {
            if let SegmentValue::CenterOf(i) = x {
                if *i < results.len() {
                    let (name, duration) = &results[*i];

                    let duration_ms = duration.as_secs_f64() * 1000.0;
                    let duration_us = duration.as_secs_f64() * 1_000_000.0;

                    let formatted_duration_ms = if duration_ms < 1.0 {
                        format!("{:.2} µs", duration_us)
                    } else {
                        format!("{:.2} ms", duration_ms)
                    };

                    format!("{}\n{}", name, formatted_duration_ms)
                } else {
                    "".to_string()
                }
            } else {
                "".to_string()
            }
        })
        .y_desc("Time Taken")
        .x_desc("Sorting Algorithm and it's Time Taken (in ms or µs)")
        .draw()
        .unwrap();

    chart
        .draw_series(
            Histogram::vertical(&chart)
                .style(RED.filled())
                .data(results.iter().enumerate().map(|(i, (_, duration))| {
                    (SegmentValue::CenterOf(i), duration.as_millis() as i32)
                })),
        )
        .unwrap();

    root_area.present().unwrap();

    if results.is_empty() {
        println!("No results to plot.");
        return;
    }

    if max_duration == 0 {
        println!("Maximum duration is zero, cannot plot.");
        return;
    }
}
