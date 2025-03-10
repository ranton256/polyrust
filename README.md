# polyrust README

## Overview

This project includes point, len, and polygon intersection algorithm implementations as a Rust learning exercise.
It can also output basic SVGs of polygons for inspection or debugging.

Here are some examples the output of the main program, showing the intersection of two convex polygons as an additional polygon.

### Figure 1
```svg
<svg width="100" height="100" xmlns="http://www.w3.org/2000/svg" viewBox="-0.1 -0.1 3.1999998 3.1999998"><polygon points="0,0 2,0 2,2 0,2 " fill="none" stroke="red" stroke-width="0.1" /><polygon points="1,1 3,1 3,3 " fill="none" stroke="blue" stroke-width="0.1" /><polygon points="1,1 2,1 2,2 " fill="none" stroke="green" stroke-width="0.1" /></svg>
```

![poly_output.svg](https://raw.githubusercontent.com/ranton256/polyrust/refs/heads/main/example_output/poly_output.svg)


### Figure 2
```svg
<svg width="300" height="200" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 5 2"><polygon points="0,0 2,0 1,2 " fill="none" stroke="blue" stroke-width="0.1" /><polygon points="0,0 4,0 5,2 " fill="none" stroke="red" stroke-width="0.1" /><polygon points="0,0 2,0 1.6666666,0.6666667 " fill="none" stroke="green" stroke-width="0.1" /></svg>
```

![Figure 2](https://raw.githubusercontent.com/ranton256/polyrust/refs/heads/main/example_output/poly_output2.svg)
