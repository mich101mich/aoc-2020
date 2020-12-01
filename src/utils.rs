#![allow(unused)]

pub use rand::prelude::*;
pub use rayon::prelude::*;
pub use std::collections::{HashMap, HashSet, VecDeque};
pub use std::io::Write;
pub use std::str::FromStr;

macro_rules! pv {
	($var: expr) => {
		println!("{}: {:?}", stringify!($var), $var)
	};
}

macro_rules! scanf {
    ( $instr:expr, $fmt:expr, $($($args:tt)::*),* ) => {
        {
            let mut res = scan_fmt::parse::scan( $instr, $fmt ) ;
            ($(scan_fmt::scan_fmt_help!(wrap res,$($args)::*).unwrap_or_else(|| panic!("Failed to parse {}", stringify!($expr)))),*)
        }
    };
}

pub fn parse(input: &str) -> i64 {
	i64::from_str(input).unwrap_or_else(|_| panic!("failed to parse >{}<", input))
}

pub trait IterExt<T> {
	fn to_vec(self) -> Vec<T>;
}
impl<T, I: Iterator<Item = T>> IterExt<T> for I {
	fn to_vec(self) -> Vec<T> {
		self.collect()
	}
}

pub fn neighbors(
	(x, y): (usize, usize),
	width: usize,
	height: usize,
) -> impl Iterator<Item = (usize, usize)> {
	let mut n = vec![];
	if y > 0 {
		n.push((x, y - 1));
	}
	if x > 0 {
		n.push((x - 1, y));
	}
	if x < width - 1 {
		n.push((x + 1, y));
	}
	if y < height - 1 {
		n.push((x, y + 1));
	}
	n.into_iter()
}

pub fn diff(a: usize, b: usize) -> usize {
	(a as isize - b as isize).abs() as usize
}
pub fn diff_i(a: isize, b: isize) -> usize {
	(a - b).abs() as usize
}

pub fn manhattan(p1: (usize, usize), p2: (usize, usize)) -> usize {
	diff(p1.0, p2.0) + diff(p1.1, p2.1)
}

pub fn manhattan_3d(p1: (usize, usize, usize), p2: (usize, usize, usize)) -> usize {
	diff(p1.0, p2.0) + diff(p1.1, p2.1) + diff(p1.2, p2.2)
}
pub fn manhattan_3d_i(p1: (isize, isize, isize), p2: (isize, isize, isize)) -> usize {
	diff_i(p1.0, p2.0) + diff_i(p1.1, p2.1) + diff_i(p1.2, p2.2)
}

pub fn get_grid<T: Clone>(value: T, width: usize, height: usize) -> Vec<Vec<T>> {
	vec![vec![value; width]; height]
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Dir {
	Up,
	Right,
	Down,
	Left,
}
pub use Dir::*;

impl Dir {
	pub fn clockwise(self) -> Dir {
		((self.num() + 1) % 4).into()
	}
	pub fn counter_clockwise(self) -> Dir {
		((self.num() + 3) % 4).into()
	}
	pub fn num(self) -> usize {
		self.into()
	}
	pub fn all() -> impl DoubleEndedIterator<Item = Dir> {
		[Up, Right, Down, Left].iter().copied()
	}
	pub fn as_delta(self) -> (isize, isize) {
		[(0, -1), (1, 0), (0, 1), (-1, 0)][self.num()]
	}
}

macro_rules! impl_from_into {
	($type:ty) => {
		impl From<$type> for Dir {
			fn from(val: $type) -> Dir {
				match val {
					0 => Up,
					1 => Right,
					2 => Down,
					3 => Left,
					n => panic!("Invalid Dir value: {}", n),
				}
			}
		}
		impl Into<$type> for Dir {
			fn into(self) -> $type {
				self as $type
			}
		}
	};
}

impl_from_into!(u8);
impl_from_into!(u16);
impl_from_into!(u32);
impl_from_into!(u64);
impl_from_into!(usize);
impl_from_into!(i8);
impl_from_into!(i16);
impl_from_into!(i32);
impl_from_into!(i64);
impl_from_into!(isize);

pub type Cost = usize;

#[derive(Debug, Clone)]
pub struct Path<P> {
	pub path: Vec<P>,
	pub cost: Cost,
}

impl<P> Path<P> {
	pub fn new(path: Vec<P>, cost: Cost) -> Path<P> {
		Path { path, cost }
	}
	pub fn append(&mut self, node: P, cost: Cost) -> &mut Self {
		self.path.push(node);
		self.cost += cost;
		self
	}
}

use std::ops::*;

impl<P> Index<usize> for Path<P> {
	type Output = P;
	fn index(&self, index: usize) -> &P {
		&self.path[index]
	}
}

pub fn ordered_insert<T, V, F>(vector: &mut Vec<T>, element: T, get_value: F)
where
	T: std::fmt::Debug,
	V: Ord,
	F: Fn(&T) -> V,
{
	let value = get_value(&element);
	let mut a = 0;
	let mut b = vector.len();
	while b - a > 1 {
		let mid = (a + b) / 2;
		if get_value(&vector[mid]) >= value {
			a = mid;
		} else {
			b = mid;
		}
	}
	vector.insert(b, element);
}

pub fn a_star_search<Id, GetNeighbors, NeighborIter, GetCost, IsWalkable, Heuristic>(
	get_all_neighbors: GetNeighbors,
	get_cost: GetCost,
	is_walkable: IsWalkable,
	start: Id,
	goal: Id,
	heuristic: Heuristic,
) -> Option<Path<Id>>
where
	Id: Copy + ::std::cmp::Eq + ::std::hash::Hash + std::fmt::Debug,
	GetNeighbors: Fn(Id) -> NeighborIter,
	NeighborIter: Iterator<Item = Id>,
	GetCost: Fn(Id, Id) -> Cost,
	Heuristic: Fn(Id) -> Cost,
	IsWalkable: Fn(Id) -> bool,
{
	if start == goal {
		return Some(Path::new(vec![start, start], 0));
	}
	let mut visited = HashMap::new();
	let mut next = vec![(start, 0)];
	visited.insert(start, (0, start));

	'search: while let Some((current_id, _)) = next.pop() {
		if current_id == goal {
			break 'search;
		}
		let current_cost = visited[&current_id].0;

		for other_id in get_all_neighbors(current_id) {
			let other_cost = current_cost + get_cost(current_id, other_id);

			if !is_walkable(other_id) && other_id != goal {
				continue;
			}

			let heuristic = heuristic(other_id);

			if let Some(&(prev_cost, _)) = visited.get(&other_id) {
				if prev_cost > other_cost {
					next.retain(|&(id, _)| id != other_id);
				}
			}

			if !visited.contains_key(&other_id) || visited[&other_id].0 > other_cost {
				ordered_insert(
					&mut next,
					(other_id, other_cost + heuristic),
					|&(_, cost)| cost,
				);
				visited.insert(other_id, (other_cost, current_id));
			}
		}
	}

	if !visited.contains_key(&goal) {
		return None;
	}

	let steps = {
		let mut steps = vec![];
		let mut current = goal;

		while current != start {
			steps.push(current);
			let (_, prev) = visited[&current];
			current = prev;
		}
		steps.push(start);
		steps.reverse();
		steps
	};

	Some(Path::new(steps, visited[&goal].0))
}

pub fn dijkstra_search<Id, GetNeighbors, NeighborIter, GetCost, IsWalkable>(
	get_all_neighbors: GetNeighbors,
	get_cost: GetCost,
	is_walkable: IsWalkable,
	start: Id,
	goals: &[Id],
) -> HashMap<Id, Path<Id>>
where
	Id: Copy + ::std::cmp::Eq + ::std::hash::Hash + ::std::fmt::Debug,
	GetNeighbors: Fn(Id) -> NeighborIter,
	NeighborIter: Iterator<Item = Id>,
	GetCost: Fn(Id, Id) -> Cost,
	IsWalkable: Fn(Id) -> bool,
{
	let mut visited = ::std::collections::HashMap::new();
	let mut next = vec![(start, 0)];
	visited.insert(start, (0, start));

	let mut remaining_goals = goals.to_vec();

	let mut goal_costs = HashMap::with_capacity(goals.len());

	while let Some((current_id, _)) = next.pop() {
		let cost = visited[&current_id].0;

		for &goal_id in remaining_goals.iter() {
			if current_id == goal_id {
				goal_costs.insert(goal_id, cost);
			}
		}
		remaining_goals.retain(|&id| id != current_id);
		if remaining_goals.is_empty() {
			break;
		}

		if !is_walkable(current_id) {
			continue;
		}

		for other_id in get_all_neighbors(current_id) {
			let other_cost = cost + get_cost(current_id, other_id);

			if !is_walkable(other_id) {
				let mut is_goal = false;
				for &goal_id in remaining_goals.iter() {
					if other_id == goal_id {
						is_goal = true;
					}
				}
				if !is_goal {
					continue;
				}
			}

			if let Some(&(prev_cost, _)) = visited.get(&other_id) {
				if prev_cost > other_cost {
					next.retain(|&(id, _)| id != other_id);
				}
			}

			if !visited.contains_key(&other_id) || visited[&other_id].0 > other_cost {
				ordered_insert(&mut next, (other_id, other_cost), |&(_, cost)| cost);
				visited.insert(other_id, (other_cost, current_id));
			}
		}
	}

	let mut goal_data = HashMap::with_capacity(goal_costs.len());

	for (&goal, &cost) in goal_costs.iter() {
		let steps = {
			let mut steps = vec![];
			let mut current = goal;

			while current != start {
				steps.push(current);
				let (_, prev) = visited[&current];
				current = prev;
			}
			steps.push(start);
			steps.reverse();
			steps
		};
		goal_data.insert(goal, Path::new(steps, cost));
	}

	goal_data
}

/// A Neighborhood for Agents moving along the 4 cardinal directions.
///
/// Also known as [Von Neumann Neighborhood](https://en.wikipedia.org/wiki/Von_Neumann_neighborhood),
/// Manhattan Metric or [Taxicab Geometry](https://en.wikipedia.org/wiki/Taxicab_geometry).
///
/// ```no_code
/// A: Agent, o: reachable in one step
///   o
///   |
/// o-A-o
///   |
///   o
/// ```
#[derive(Clone, Copy, Debug)]
pub struct ManhattanNeighborhood {
	width: usize,
	height: usize,
}

impl ManhattanNeighborhood {
	/// Creates a new ManhattanNeighborhood.
	///
	/// `width` and `height` are the size of the Grid to move on.
	pub fn new(width: usize, height: usize) -> ManhattanNeighborhood {
		ManhattanNeighborhood { width, height }
	}
	pub fn get_all_neighbors(
		&self,
		point: (usize, usize),
	) -> Box<dyn Iterator<Item = (usize, usize)>> {
		let (width, height) = (self.width, self.height);

		let iter = [(0isize, -1isize), (1, 0), (0, 1), (-1, 0)]
			.iter()
			.map(move |(dx, dy)| (point.0 as isize + dx, point.1 as isize + dy))
			.filter(move |(x, y)| {
				*x >= 0 && *y >= 0 && (*x as usize) < width && (*y as usize) < height
			})
			.map(|(x, y)| (x as usize, y as usize));

		Box::new(iter)
	}
	pub fn heuristic(&self, point: (usize, usize), goal: (usize, usize)) -> usize {
		let diff_0 = if goal.0 > point.0 {
			goal.0 - point.0
		} else {
			point.0 - goal.0
		};
		let diff_1 = if goal.1 > point.1 {
			goal.1 - point.1
		} else {
			point.1 - goal.1
		};
		diff_0 + diff_1
	}
}
