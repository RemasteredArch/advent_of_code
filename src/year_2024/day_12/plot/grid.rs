use std::collections::HashMap;

use crate::Integer;

use super::places::{AddError, Coordinates, Direction, Plant, Span};

#[derive(Clone, Hash, Debug, PartialEq, Eq)]
struct Grid<'a> {
    plot: Box<[Box<[Plant]>]>,
    original: &'a [Box<[Plant]>],
}

impl<'a> Grid<'a> {
    pub fn new(grid: &'a [Box<[Plant]>]) -> Self {
        Self {
            plot: grid.into(),
            original: grid,
        }
    }

    fn get_impl(grid: &[Box<[Plant]>], coordinates: Coordinates) -> Option<Plant> {
        grid.get(coordinates.row)?.get(coordinates.column).copied()
    }

    fn get(&self, coordinates: Coordinates) -> Option<Plant> {
        Self::get_impl(&self.plot, coordinates)
    }

    fn get_mut(&mut self, coordinates: Coordinates) -> Option<&mut Plant> {
        self.plot
            .get_mut(coordinates.row)?
            .get_mut(coordinates.column)
    }

    fn null(&mut self, coordinates: Coordinates) {
        if let Some(plant) = self.get_mut(coordinates) {
            *plant = Plant::NULL;
        }
    }
}

#[derive(Clone, Hash, Debug, PartialEq, Eq)]
pub struct StandardGrid<'a> {
    grid: Grid<'a>,
    regions: Vec<(Integer, Integer)>,
}

impl<'a> StandardGrid<'a> {
    pub fn new(grid: &'a [Box<[Plant]>]) -> Self {
        Self {
            grid: Grid::new(grid),
            regions: vec![],
        }
    }

    pub fn regions(&self) -> &[(Integer, Integer)] {
        &self.regions
    }

    pub fn visit(&mut self, coordinates: Coordinates) {
        let plant = match self.grid.get(coordinates) {
            Some(plant) if plant != Plant::NULL => plant,
            _ => return,
        };

        self.regions.push((0, 0));
        self.visit_impl(plant, coordinates);
    }

    /// Returns `true` if the plant at the `coordinates` matches `region_type`.
    ///
    /// Adds to [`Self::regions`].
    fn visit_impl(&mut self, region_type: Plant, coordinates: Coordinates) -> bool {
        // Escape if plant at `coordinates` is non-matching, otherwise mark it as visited
        // and proceed.
        match self.grid.get(coordinates) {
            // Matching and unvisited plant, continue.
            Some(plant) if plant == region_type => (),
            // Visited plant; return `true` if it was previously matching, but do not continue.
            Some(plant) if plant == Plant::NULL => {
                return Grid::get_impl(self.grid.original, coordinates)
                    .is_some_and(|plant| plant == region_type);
            }
            // No plant or non-matching plant, return `false`.
            _ => return false,
        }

        self.grid.null(coordinates);

        let non_matching_edges = Direction::all()
            .iter()
            .filter(|&&edge| {
                let next_coordinates = match coordinates.step(edge) {
                    Ok(next_coordinates) => next_coordinates,
                    Err(AddError::OutOfBounds) => return true,
                    Err(AddError::Overflow) => {
                        panic!("overflowed while attempted to advance coordinates")
                    }
                };

                !self.visit_impl(region_type, next_coordinates)
            })
            .count();

        let region = self
            .regions
            .last_mut()
            .expect("`Self::visit` includes a `push`");

        *region = (
            // Area
            region.0 + 1,
            // Perimeter
            region.1 + non_matching_edges as Integer,
        );

        true
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BulkGrid<'a> {
    grid: Grid<'a>,
    /// Stores a list of contiguous sections of the same plant.
    ///
    /// The [`Integer`] represents area. The [`HashMap`] stores [`Coordinates`] along the edges of
    /// the region, with a list of which edges ([`Direction`]) of that location is exposed.
    regions: Vec<(Integer, HashMap<Coordinates, Vec<Direction>>)>,
}

impl<'a> BulkGrid<'a> {
    pub fn new(grid: &'a [Box<[Plant]>]) -> Self {
        Self {
            grid: Grid::new(grid),
            regions: vec![],
        }
    }

    pub fn visit(&mut self, coordinates: Coordinates) {
        let plant = match self.grid.get(coordinates) {
            Some(plant) if plant != Plant::NULL => plant,
            _ => return,
        };

        self.regions.push((0, HashMap::new()));
        self.visit_impl(plant, coordinates);
    }

    /// Returns `true` if the plant at the `coordinates` matches `region_type`.
    ///
    /// Adds to [`Self::regions`].
    fn visit_impl(&mut self, region_type: Plant, coordinates: Coordinates) -> bool {
        // Escape if plant at `coordinates` is non-matching, otherwise mark it as visited
        // and proceed.
        match self.grid.get(coordinates) {
            // Matching and unvisited plant, continue.
            Some(plant) if plant == region_type => (),
            // Visited plant; return `true` if it was previously matching, but do not continue.
            Some(plant) if plant == Plant::NULL => {
                return Grid::get_impl(self.grid.original, coordinates)
                    .is_some_and(|plant| plant == region_type);
            }
            // No plant or non-matching plant, return `false`.
            _ => return false,
        }
        self.grid.null(coordinates);

        let non_matching_edges: Vec<_> = Direction::all()
            .into_iter()
            .filter(|&edge| {
                let next_coordinates = match coordinates.step(edge) {
                    Ok(next_coordinates) => next_coordinates,
                    Err(AddError::OutOfBounds) => return true,
                    Err(AddError::Overflow) => {
                        panic!("overflowed while attempted to advance coordinates")
                    }
                };

                !self.visit_impl(region_type, next_coordinates)
            })
            .collect();

        let region = self
            .regions
            .last_mut()
            .expect("`Self::visit` includes a `push`");

        // Area
        //
        // Does this actually set it?
        region.0 += 1;

        if non_matching_edges.is_empty() {
            return true;
        }

        match region.1.get_mut(&coordinates) {
            Some(edges) => {
                for edge in non_matching_edges {
                    if !edges.contains(&edge) {
                        edges.push(edge);
                    }
                }
            }
            None => {
                region.1.insert(coordinates, non_matching_edges);
            }
        }

        true
    }

    /// Transforms [`Self`] into a vector holding the area and number of edges for every region.
    pub fn into_regions(self) -> Vec<(Integer, Integer)> {
        fn fuse(spans: &mut Vec<Span>) -> bool {
            let mut fused = false;

            for i in spans.len()..0 {
                let popped_span = spans[i];

                if spans
                    .iter_mut()
                    .any(|span| span.join(popped_span).is_some())
                {
                    spans.remove(i);
                    fused = true;
                }
            }

            fused
        }

        let mut regions: Vec<(Integer, Integer)> = vec![];

        for (area, exposed_locations) in self.regions {
            let mut spans = Vec::<Span>::new();

            for (coordinates, exposed_edges) in exposed_locations {
                for edge in exposed_edges {
                    let was_inserted = spans.iter_mut().any(|span| {
                        if span.exposed_edge() == edge {
                            return false;
                        }

                        span.append(coordinates).is_some()
                    });

                    if !was_inserted {
                        spans.push(Span::new_no_run(coordinates, edge));
                    }
                }
            }

            while fuse(&mut spans) {}

            let perimeter = spans.iter().map(|span| span.len() as Integer).sum();

            regions.push((area, perimeter));
        }

        regions
    }
}
