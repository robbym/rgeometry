use crate::array::Turn;
use crate::point::Point;
use crate::{ConvexPolygon, Error, Polygon};
use crate::{PolygonScalar, PolygonScalarRef};

// https://en.wikipedia.org/wiki/Graham_scan

// O(n log n)
// Doesn't allocate.
// Properties:
//    No panics.
//    All Ok results are valid convex polygons.
//    No points are outside the resulting convex polygon.
pub fn convex_hull<T>(mut pts: Vec<Point<T, 2>>) -> Result<ConvexPolygon<T>, Error>
where
  T: PolygonScalar,
  for<'a> &'a T: PolygonScalarRef<&'a T, T>,
{
  let smallest: &Point<T, 2> = &smallest_point(&pts)?;

  pts.sort_unstable_by(|a, b| {
    smallest
      .ccw_cmp_around(a, b)
      .then_with(|| smallest.cmp_distance_to(a, b))
  });
  let mut known_good = 2;
  let mut at = 2;
  while at < pts.len() {
    if at != known_good {
      pts.swap(at, known_good);
    }
    let p1 = &pts[known_good];
    let p2 = &pts[known_good - 1];
    let p3 = &pts[known_good - 2];
    match p3.turn(p2, p1) {
      Turn::CounterClockWise => {
        at += 1;
        known_good += 1;
      }
      Turn::ClockWise | Turn::CoLinear => {
        pts.swap(at, known_good - 1);
        at += 1;
      }
    }
  }
  pts.truncate(known_good);
  Ok(ConvexPolygon(Polygon::new(pts)))
}

// Find the smallest point and remove it from the vector
// O(n)
fn smallest_point<T>(pts: &[Point<T, 2>]) -> Result<Point<T, 2>, Error>
where
  T: PolygonScalar,
  for<'a> &'a T: PolygonScalarRef<&'a T, T>,
{
  Ok(
    pts
      .iter()
      .min_by(|a, b| {
        a.y_coord()
          .partial_cmp(b.y_coord())
          .unwrap()
          .then_with(|| a.x_coord().partial_cmp(b.x_coord()).unwrap())
      })
      .ok_or(Error::InsufficientInput)?
      .clone(),
  )
}
