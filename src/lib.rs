//! Squarified Treemap Layout
//!
//! Implements the algorithm from Bruls, Huizing, van Wijk, "Squarified Treemaps"
//!
//! This is a direct translation of the python implementation by
//! [here](https://github.com/laserson/squarify).

/// Represents a rectangle with an x and y coordinate,
/// as well as a width (dx) and height (dy)
#[derive(Debug, Clone)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub dx: f64,
    pub dy: f64,
}

impl Rect {
    /// Create a new rectangle with coordinates x and y,
    /// and dimensions dx and dy
    pub fn new(x: f64, y: f64, dx: f64, dy: f64) -> Self {
        Self { x, y, dx, dy }
    }
}

/// From a list of values, return the treemap layout. This layout is a list of rectangles,
/// each describing its x coordinate, y coordinate, width (dx), and height (dy).
///
/// Note that this function already takes care of normalizing values to the size of the rectangle.
///
/// You can optionally specify padding to allow some empty space between rectangles (more appealing
/// visually).
///
/// # Example
///
/// ```
/// use squarify::squarify;
///
/// fn example() {
///     
///     let sizes = vec![24.0, 25.0, 100.0, 99.7, 56.2, 32.1];
///
///     // Let's create a layout starting at coordinate 0, 0 (usually what we want),
///     // with height 1000.0 and width 1000.0
///     let x = 0.0;
///     let y = 0.0;
///     let width = 1000.0;
///     let height = 1000.0;
///
///     // Get rectangles without padding
///     let rects = squarify(&sizes, x, y, width, height, None);
///
///     // Or add some padding
///     let padded_rects = squarify(&sizes, x, y, width, height, 2.0);
///
/// }
/// ```
pub fn squarify<I>(sizes: &[f64], x: f64, y: f64, dx: f64, dy: f64, padding: I) -> Vec<Rect>
where
    I: Into<Option<f64>>,
{
    if sizes.is_empty() {
        return Vec::with_capacity(0);
    } else if sizes.len() == 1 {
        return vec![Rect::new(x, y, dx, dy)];
    }

    let sizes = normalized_sizes(sizes, dx, dy);
    let mut rects = _squarify(&sizes, x, y, dx, dy);

    if let Some(pad) = padding.into() {
        for rect in rects.iter_mut() {
            if rect.dx > pad {
                rect.x += pad / 2.0;
                rect.dx -= pad;
            }
            if rect.dy > pad {
                rect.y += pad / 2.0;
                rect.dy -= pad;
            }
        }
    }

    rects
}

fn _squarify(sizes: &[f64], x: f64, y: f64, dx: f64, dy: f64) -> Vec<Rect> {
    if sizes.is_empty() {
        return Vec::with_capacity(0);
    } else if sizes.len() == 1 {
        return layout(sizes, x, y, dx, dy);
    }

    let mut idx = 1;
    while idx < sizes.len()
        && worst_ratio(&sizes[..idx], x, y, dx, dy) >= worst_ratio(&sizes[..=1], x, y, dx, dy)
    {
        idx += 1;
    }

    let current = &sizes[..idx];
    let remaining = &sizes[idx..];
    let lover = leftover(current, x, y, dx, dy);

    let mut finished = layout(current, x, y, dx, dy);
    let rest = _squarify(remaining, lover.x, lover.y, lover.dx, lover.dy);
    finished.extend(rest);
    finished
}

fn normalized_sizes(sizes: &[f64], dx: f64, dy: f64) -> Vec<f64> {
    let total_size: f64 = sizes.iter().sum();
    let total_area = dx * dy;
    sizes.iter().map(|x| x * total_area / total_size).collect()
}

fn layout(sizes: &[f64], x: f64, y: f64, dx: f64, dy: f64) -> Vec<Rect> {
    if dx >= dy {
        layout_row(sizes, x, y, dx, dy)
    } else {
        layout_col(sizes, x, y, dx, dy)
    }
}

fn layout_row(sizes: &[f64], x: f64, mut y: f64, _dx: f64, dy: f64) -> Vec<Rect> {
    let covered_area: f64 = sizes.iter().sum();
    let width = covered_area / dy;

    let mut rects = Vec::with_capacity(sizes.len());
    for size in sizes {
        let rect = Rect::new(x, y, width, size / width);
        rects.push(rect);
        y += size / width;
    }

    rects
}

fn layout_col(sizes: &[f64], mut x: f64, y: f64, dx: f64, _dy: f64) -> Vec<Rect> {
    let covered_area: f64 = sizes.iter().sum();
    let height = covered_area / dx;
    let mut rects = Vec::with_capacity(sizes.len());
    for size in sizes {
        let rect = Rect::new(x, y, size / height, height);
        rects.push(rect);
        x += size / height;
    }

    rects
}

fn leftover(sizes: &[f64], x: f64, y: f64, dx: f64, dy: f64) -> Rect {
    if dx >= dy {
        leftover_row(sizes, x, y, dx, dy)
    } else {
        leftover_col(sizes, x, y, dx, dy)
    }
}

fn leftover_row(sizes: &[f64], x: f64, y: f64, dx: f64, dy: f64) -> Rect {
    let covered_area: f64 = sizes.iter().sum();
    let width = covered_area / dy;
    Rect::new(x + width, y, dx - width, dy)
}

fn leftover_col(sizes: &[f64], x: f64, y: f64, dx: f64, dy: f64) -> Rect {
    let covered_area: f64 = sizes.iter().sum();
    let height = covered_area / dx;
    Rect::new(x, y + height, dx, dy - height)
}

fn worst_ratio(sizes: &[f64], x: f64, y: f64, dx: f64, dy: f64) -> f64 {
    layout(sizes, x, y, dx, dy)
        .into_iter()
        .map(|x| f64::max(x.dx / x.dy, x.dy / x.dx))
        .fold(std::f64::NAN, f64::max)
}

#[cfg(test)]
mod tests {
    use super::*;

    const FLOAT_MARGIN_OF_ERROR: f64 = 0.000001;

    /// Test mirrored from
    /// https://github.com/laserson/squarify/blob/master/tests/test_squarify.py
    #[test]
    fn test_squarify() {
        let values = vec![500.0, 433.0, 78.0, 25.0, 25.0, 7.0];

        let expected = vec![
            Rect::new(0.0, 0.0, 327.7153558052434, 433.0),
            Rect::new(327.7153558052434, 0.0, 372.2846441947566, 330.0862676056338),
            Rect::new(
                327.7153558052434,
                330.0862676056338,
                215.0977944236371,
                102.9137323943662,
            ),
            Rect::new(
                542.8131502288805,
                330.0862676056338,
                68.94160077680677,
                102.9137323943662,
            ),
            Rect::new(
                611.7547510056874,
                330.0862676056338,
                88.24524899431273,
                80.40135343309854,
            ),
            Rect::new(
                611.7547510056874,
                410.4876210387323,
                88.2452489943124,
                22.51237896126767,
            ),
        ];
        let observed = squarify(&values, 0.0, 0.0, 700.0, 433.0, None);
        assert_eq!(expected.len(), observed.len());
        for (o, e) in observed.into_iter().zip(expected.into_iter()) {
            assert!((o.x - e.x).abs() < FLOAT_MARGIN_OF_ERROR);
            assert!((o.y - e.y).abs() < FLOAT_MARGIN_OF_ERROR);
            assert!((o.dx - e.dx).abs() < FLOAT_MARGIN_OF_ERROR);
            assert!((o.dy - e.dy).abs() < FLOAT_MARGIN_OF_ERROR);
        }
    }
}
