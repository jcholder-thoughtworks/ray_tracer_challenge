use ray_tracer_challenge::*;
use ray_tracer_challenge::math::transforms::*;

fn main() {
    // Scenario: Scaling a ray
    //   Given r ← ray(point(1, 2, 3), vector(0, 1, 0))
    //     And m ← scaling(2, 3, 4)
    //   When r2 ← transform(r, m)
    //   Then r2.origin = point(2, 6, 12)
    //     And r2.direction = vector(0, 3, 0)

    let r = Ray::new(Point::new(1.0, 2.0, 3.0), Vector::new(0.0, 1.0, 0.0));
    let m = scaling(2.0, 3.0, 4.0);

    let r2 = r.transform(&m);

    assert_eq!(Point::new(2.0, 6.0, 12.0), r2.origin);
    assert_eq!(Vector::new(0.0, 3.0, 0.0), r2.direction);

    println!("{:?} -> {:?}", r, r2);
}
