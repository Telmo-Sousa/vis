// Bubble Sort
pub fn bubble_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 0..n {
        let mut swapped = false;
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}

// Cocktail Shaker Sort
pub fn cocktail_shaker_sort(arr: &mut [i32]) {
    let mut swapped = true;
    let mut start = 0;
    let mut end = arr.len() - 1;

    while swapped {
        swapped = false;

        // Forward pass
        for i in start..end {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }
        swapped = false;
        end -= 1;

        // Backward pass
        for i in (start..end).rev() {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swapped = true;
            }
        }

        start += 1;
    }
}

// Cycle Sort
pub fn cycle_sort(arr: &mut [i32]) {
    let n = arr.len();

    for cycle_start in 0..n - 1 {
        let mut item = arr[cycle_start];

        // Find position where we put the item
        let mut pos = cycle_start;
        for i in cycle_start + 1..n {
            if arr[i] < item {
                pos += 1;
            }
        }

        // If item is already in correct position
        if pos == cycle_start {
            continue;
        }

        // Ignore duplicates
        while item == arr[pos] {
            pos += 1;
        }

        // Put the item to its right position
        if pos != cycle_start {
            std::mem::swap(&mut item, &mut arr[pos]);
        }

        // Rotate the rest of the cycle
        while pos != cycle_start {
            pos = cycle_start;

            // Find position where we put the item
            for i in cycle_start + 1..n {
                if arr[i] < item {
                    pos += 1;
                }
            }

            // Ignore duplicates
            while item == arr[pos] {
                pos += 1;
            }

            // Put the item to its right position
            if item != arr[pos] {
                std::mem::swap(&mut item, &mut arr[pos]);
            }
        }
    }
}

// Gnome Sort
pub fn gnome_sort(arr: &mut [i32]) {
    let n = arr.len();
    let mut i = 0;

    while i < n {
        if i == 0 || arr[i - 1] <= arr[i] {
            i += 1;
        } else {
            arr.swap(i, i - 1);
            i -= 1;
        }
    }
}

// Heap Sort
pub fn heap_sort(arr: &mut [i32]) {
    let n = arr.len();

    // Build max heap
    for i in (0..n / 2).rev() {
        heapify(arr, n, i);
    }

    // Extract elements from heap one by one
    for i in (1..n).rev() {
        arr.swap(0, i);
        heapify(arr, i, 0);
    }
}

fn heapify(arr: &mut [i32], n: usize, root: usize) {
    let mut largest = root;
    let left = 2 * root + 1;
    let right = 2 * root + 2;

    if left < n && arr[left] > arr[largest] {
        largest = left;
    }

    if right < n && arr[right] > arr[largest] {
        largest = right;
    }

    if largest != root {
        arr.swap(root, largest);
        heapify(arr, n, largest);
    }
}

// Insertion Sort
pub fn insertion_sort(arr: &mut [i32]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}

// Merge Sort
pub fn merge_sort(arr: &mut [i32]) {
    let n = arr.len();
    if n <= 1 {
        return;
    }

    let mid = n / 2;
    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);

    let mut merged = Vec::with_capacity(n);
    let mut left = 0;
    let mut right = mid;

    while left < mid && right < n {
        if arr[left] <= arr[right] {
            merged.push(arr[left]);
            left += 1;
        } else {
            merged.push(arr[right]);
            right += 1;
        }
    }

    merged.extend_from_slice(&arr[left..mid]);
    merged.extend_from_slice(&arr[right..n]);

    arr.copy_from_slice(&merged);
}

// Quicksort
pub fn quicksort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }

    let pivot_index = partition(arr);

    quicksort(&mut arr[0..pivot_index]);
    quicksort(&mut arr[pivot_index + 1..]);
}

fn partition(arr: &mut [i32]) -> usize {
    let len = arr.len();
    let pivot_index = len - 1;
    let pivot = arr[pivot_index];

    let mut i = 0;

    for j in 0..len - 1 {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, pivot_index);
    i
}

// Selection Sort
pub fn selection_sort(arr: &mut [i32]) {
    let n = arr.len();

    for i in 0..n {
        let mut min_idx = i;

        for j in i + 1..n {
            if arr[j] < arr[min_idx] {
                min_idx = j;
            }
        }

        arr.swap(i, min_idx);
    }
}

// Shell Sort
pub fn shell_sort(arr: &mut [i32]) {
    let n = arr.len();
    let mut gap = n / 2;

    while gap > 0 {
        for i in gap..n {
            let temp = arr[i];
            let mut j = i;

            while j >= gap && arr[j - gap] > temp {
                arr[j] = arr[j - gap];
                j -= gap;
            }

            arr[j] = temp;
        }

        gap /= 2;
    }
}

