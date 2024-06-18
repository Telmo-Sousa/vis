# vis 🦀

A Sorting Algorithm Performance Visualizer written in Rust. It provides a comparative analysis of various sorting algorithms by timing their execution on a reversed array of 10,000 integers and visualizing the results in a histogram.

## Features

- Implements multiple sorting algorithms:
  - Bubble Sort
  - Cocktail Shaker Sort
  - Cycle Sort
  - Gnome Sort
  - Heap Sort
  - Insertion Sort
  - Merge Sort
  - Quicksort
  - Selection Sort
  - Shellsort
- Generates a histogram illustrating execution times of each algorithm.
- Each algorithm's execution time is displayed in milliseconds or microseconds.
- Helps in evaluating and comparing sorting algorithm efficiency for different dataset scenarios.

## Requirements

- Rust programming language installed ([Install Rust](https://www.rust-lang.org/))

## Usage

1. Ensure Rust is installed on your system.
2. Clone the repository:
    ```bash
    git clone https://github.com/Telmo-Sousa/vis.git
    cd vis
    ```
3. Run the project:
    ```bash
    cargo run --release
    ```

## Warnings

- Initial execution times may vary due to factors like system load or compilation artifacts. Run the project multiple times for accurate comparisons.

## FAQ

- **Why does it generate an image?**
  - The image visualizes sorting algorithm performance, making it easier to understand and compare their efficiency on large datasets.

## Example

![Sorting Performance](/sort_performance.png)

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
