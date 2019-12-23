extern crate num;
use num::complex::Complex;
use std::fs::File;
use std::io::{Write, Error, ErrorKind};
use std::f32::consts::PI;
use std::fmt::Write as Ww;
use std::ops::{AddAssign};

type Cmp = Complex<f32>;

struct Polygon {
    n: u32,
    center: Cmp,
    length: f32,
    angle: f32,
}

struct Square {
    center: Cmp,
    length: f32,
}

struct Circle {
    center: Cmp,
    radius: f32,
}

fn main() {
    let file: File = File::create("L10").unwrap();
    let mut line_counter: u32 = 0;
    let is_test: bool = false;

    if is_test {
        let sqr: Square = Square {
            center: Cmp::new(12.5e0, 12.5e0),
            length: 25e0
        };
        let pent: Polygon = Polygon{
            n: 5,
            center: Cmp::new(3e0, 10e0),
            length: 4.359255,
            angle: 0.0
        };
        do_square(&file, sqr, &mut line_counter).unwrap();
        do_polygon(&file, pent, &mut line_counter).unwrap();
    }
    else {
        let offset: Cmp = Cmp::new(25e0, 25e0);
        let c_pent: Cmp = Cmp::new(7.441, 0e0);
        let c_hex: Cmp = Cmp::new(18.39, 0e0);
        let c_cir: Cmp = Cmp::new(10.404, -10.404);
        let c_sqr_1: Cmp = Cmp::new(20.25, -11.45);
        let c_sqr_2: Cmp = Cmp::new(11.45, -20.25);
        let c_sqr_3: Cmp = Cmp:: new(0e0, 0e0);

        let l_pent: f32 = 5e0;
        let l_hex: f32 = 5.9e0;
        let r_cir: f32 = 5e0;
        let l_sqr: f32 = 5e0;

        let a_pent: f32 = 0e0;
        let a_hex: f32 = 30e0;

        let sqr_3: Square = Square{ center: offset + c_sqr_3, length: l_sqr };

        assert!(setup(&file, &mut line_counter).is_ok());

        lift_secure(&file, &mut line_counter).unwrap();

        for i in 0..4 {
            let ang: f32 = (i as f32) * 90e0;
            let ang_rad: f32 = ang * PI / 180e0;
            let rot: Cmp = Cmp::from_polar(&(1e0), &ang_rad);

            let pent: Polygon = Polygon {
                n: 5,
                center: offset + c_pent * rot,
                length: l_pent,
                angle: a_pent + ang
            };
            let hex: Polygon = Polygon{
                n: 6,
                center: offset + c_hex * rot,
                length: l_hex,
                angle: a_hex + ang
            };

            let cir:Circle = Circle{ center: offset + c_cir * rot, radius: r_cir };

            let sqr_1: Square = Square{ center: offset + c_sqr_1 * rot, length: l_sqr };
            let sqr_2: Square = Square{ center: offset + c_sqr_2 * rot, length: l_sqr };

            do_square(&file, sqr_2, &mut line_counter).unwrap();
            do_circle(&file, cir, &mut line_counter).unwrap();
            do_square(&file, sqr_1, &mut line_counter).unwrap();
            do_polygon(&file, pent, &mut line_counter).unwrap();
            do_polygon(&file, hex, &mut line_counter).unwrap();
        }

        do_square(&file, sqr_3, &mut line_counter).unwrap();

        assert!(end(&file, &mut line_counter).is_ok());
    }


    println!("Hello, world!");
}

// Drill empty square.
fn do_square(file: &File, square: Square, lc: &mut u32) -> std::io::Result<()>{
    return drill_square(file, square.length, square.center, lc);
}

fn drill_square(mut file: &File, length: f32, center: Cmp, lc: &mut u32) -> std::io::Result<()>{
    let r01: String = String::from("R01=1");
    let r02: String = String::from("R02=0.");
    let r03: String = String::from("R03=1");
    let r12: String = String::from(format!("R12={0:0<8.3}", length));
    let r13: String = String::from(format!("R13={0:0<8.3}", length));
    let r15: String = String::from(format!("R15={0:0<8.3}", 80.));
    let r16: String = String::from(format!("R16={0:0<8.3}", 80.));
    let r22: String = String::from(format!("R22={0:0<8.3}", center.re));
    let r23: String = String::from(format!("R23={0:0<8.3}", center.im));
    let r24: String = String::from(format!("R24={0:0<3.1}", 1.5));
    *lc = (*lc / 100 + 1) * 100;

    positioning(file, &center, lc).unwrap();
    let operation: String = format!(
        "N{10} {0} {1} {2} {3} {4} {5} {6} {7} {8} {9} L903\r\n",
        r01, r02, r03, r12, r13, r15, r16, r22, r23, r24, lc
    );

    file.write_all(operation.as_bytes()).unwrap();
    return lift_secure(&file, lc);
}

// Drill empty circle.
fn do_circle(file: &File, circle: Circle, lc: &mut u32) -> std::io::Result<()>{
    return drill_circle(file, circle.radius, circle.center, lc);
}

fn drill_circle(mut file: &File, radius: f32, center: Cmp, lc: &mut u32) -> std::io::Result<()>{
    let r01: String = String::from("R01=1");
    let r02: String = String::from("R02=0.");
    let r03: String = String::from("R03=1");
    let r06: String = String::from("R06=02");
    let r15: String = String::from(format!("R15={0:0<8.3}", 80.));
    let r16: String = String::from(format!("R16={0:0<8.3}", 30.));
    let r22: String = String::from(format!("R22={0:0<8.3}", center.re));
    let r23: String = String::from(format!("R23={0:0<8.3}", center.im));
    let r24: String = String::from(format!("R24={0:0<8.3}", radius));

    *lc = (*lc / 100 + 1) * 100;
    positioning(file, &center, lc).unwrap();

    let operation: String = format!(
        "N{9} {0} {1} {2} {3} {4} {5} {6} {7} {8} L930\r\n",
        r01, r02, r03, r06, r15, r16, r22, r23, r24, lc
    );

    file.write_all(operation.as_bytes()).unwrap();
    return lift_secure(&file, lc);
}

// Drill regular polygon of n-vertex and length-sides.
fn do_polygon(file: &File, polygon: Polygon, lc: &mut u32) -> std::io::Result<()>{
    return drill_rec_polygon(
        file,
        polygon.n,
        polygon.length,
        polygon.center,
        polygon.angle,
        lc);
}

fn drill_rec_polygon(file: &File, n: u32, length: f32, center: Cmp, offset: f32, lc: &mut u32) -> std::io::Result<()> {
    if (n < 3) || (n > 9) {
        return Err(Error::new(ErrorKind::InvalidInput, "asa"));
    }

    // Some parameters
    let alpha: f32 = 360e0 / (n as f32);
    let beta: f32 = 180e0 - alpha;
    let alpha_half_rad: f32 = alpha / 2e0 * PI / 180e0;
    let beta_half_rad: f32 = beta / 2e0 * PI / 180e0;

    let vertex_distance: f32 = length / 2e0 / alpha_half_rad.sin();
    let apothem: f32 = length / 2e0 / alpha_half_rad.tan();
    let d_max: f32 = vertex_distance * (1e0 - alpha_half_rad.cos());
    let n_perimeter: u32 = (d_max / 3e0).ceil() as u32;

    let first_distance: f32 = vertex_distance - d_max + 1.5e0;
    let first_v0: Cmp = center + Cmp::from_polar(&first_distance, &((offset) * PI / 180e0));
    let first_vn: Cmp = center + Cmp::from_polar(&first_distance, &((offset - alpha) * PI / 180e0));
    let first_a0: Cmp = (first_v0 + first_vn) * 5e-1;

    let last_distance: f32 = vertex_distance - 1.5e0 / beta_half_rad.sin();
    let last_v0: Cmp = center + Cmp::from_polar(&last_distance, &((offset) * PI / 180e0));
    let last_vn: Cmp = center + Cmp::from_polar(&last_distance, &((offset - alpha) * PI / 180e0));
    let last_a0: Cmp = (last_v0 + last_vn) * 5e-1;
    println!("{}", &last_a0);
    let end_a0: Cmp = center + (
        Cmp::from_polar(&vertex_distance, &((offset) * PI / 180e0)) +
            Cmp::from_polar(&vertex_distance, &((offset - alpha) * PI / 180e0))
    ) * 5e-1;

    // Move to center.
    // Unnecessary, circular drill makes it automatically.
    //move_unhindered(&file, &center).unwrap();

    // Drill inner circle.
    match drill_circle(file, apothem, center, lc) {
        Err(e) => return Err(e),
        Ok(o) => o
    }

    // Drill until contour.
    if n_perimeter > 1 {
        move_unhindered(file, &first_a0, lc).unwrap();
    }
    for i in 1..n_perimeter {
        let distance: f32 = vertex_distance - d_max + 3e0 * ((i - 1) as f32) + 1.5e0;
        let v0: Cmp = center + Cmp::from_polar(&distance, &((offset) * PI / 180e0));
        let vn: Cmp = center + Cmp::from_polar(&distance, &((offset - alpha) * PI / 180e0));
        let a0: Cmp = (v0 + vn) * 5e-1;

        if i > 1 { drill_line(file, &a0, lc).unwrap(); } // Go to next apothem.

        for j in 0..n {
            let angle_next: f32 = (offset + (j as f32) * alpha) * PI / 180e0;
            let v_next: Cmp = center + Cmp::from_polar(&distance, &angle_next);
            drill_line(file, &v_next, lc).unwrap();  // Go from vertex to vertex.
        }

        drill_line(file, &a0, lc).unwrap();  // Return to first apothem.
    }

    // Drill contour.
    {
        drill_point(file, &last_a0, lc).unwrap();
        // drill_line(file, &last_a0, lc).unwrap(); // Drill to last apothem without contour knowledge.
        set_contour(file, lc).unwrap(); // Set contour operations on.
        for j in 0..n {
            let angle_next: f32 = (offset + (j as f32) * alpha) * PI / 180e0;
            let v_next: Cmp = center + Cmp::from_polar(&vertex_distance, &angle_next);
            drill_line(file, &v_next, lc).unwrap();  // Go from vertex to vertex within contour.
        }
        drill_line(file, &end_a0, lc).unwrap(); // Drill to last apothem.
        unset_contour(file, lc).unwrap(); // Unset contour operations.
    }

    // Lift to secure.
    lift_secure(&file, lc).unwrap();

    return Ok(());
}

// Set contour operations on.
fn set_contour(mut file: &File, lc: &mut u32) -> std::io::Result<()>{
    *lc = (*lc / 100) * 100 + (((*lc % 100) / 10) + 1) * 10;
    let operation: String = format!("N{0} G41\r\n", lc);
    return file.write_all(operation.as_bytes());
}

// Set contour operations off.
fn unset_contour(mut file: &File, lc: &mut u32) -> std::io::Result<()>{
    *lc = (*lc / 100) * 100 + (((*lc % 100) / 10) + 1) * 10;
    let operation: String = format!("N{0} G40\r\n", lc);
    return file.write_all(operation.as_bytes());
}

// Lift to secure zone.
fn lift_secure(mut file: &File, lc: &mut u32) -> std::io::Result<()> {
    lc.add_assign(1);
    let operation: String = format!("N{0} G1 Z0.5 F80\r\n", lc);
    return file.write_all(operation.as_bytes());
}

// Lift, move and drill tool onto a point.
fn move_unhindered(mut file: &File, p_fin: &Cmp, lc: &mut u32) -> std::io::Result<()>{
    // let operation_lift: String = format!("G1 Z0.5 F80\r\n");
    lc.add_assign(1);
    let operation_move: String = format!(
        "N{2} G0 X{0:0<8.3} Y{1:0<8.3} F80\r\n", p_fin.re, p_fin.im, lc
    );
    lc.add_assign(1);
    let operation_drill: String = format!("N{0} G1 Z-1. F30\r\n", lc);

    let mut operation: String = String::new();
    // operation.write_str(operation_lift.as_str()).unwrap();
    operation.write_str(operation_move.as_str()).unwrap();
    operation.write_str(operation_drill.as_str()).unwrap();

    return file.write_all(operation.as_bytes());
}

// Position tool above
fn positioning(mut file: &File, p_fin: &Cmp, lc: &mut u32) -> std::io::Result<()> {
    // let operation_lift: String = format!("G1 Z0.5 F80\r\n");
    lc.add_assign(1);
    let operation: String = format!(
        "N{2} G0 X{0:0<8.3} Y{1:0<8.3} F80\r\n", p_fin.re, p_fin.im, lc
    );

    return file.write_all(operation.as_bytes());
}

// Move and down to point.
fn drill_point(mut file: &File, p_fin: &Cmp, lc: &mut u32) -> std::io::Result<()>{
    lc.add_assign(1);
    let operation_move: String = format!(
        "N{2} G1 X{0:0<8.3} Y{1:0<8.3} F80\r\n", p_fin.re, p_fin.im, lc
    );
    lc.add_assign(1);
    let operation_down: String = format!(
        "N{0} G1 Z-1 F30\r\n", lc
    );
    let operation: String = operation_move + operation_down.as_str();
    return file.write_all(operation.as_bytes());
}

// Linear drill.
fn drill_line(mut file: &File, p_fin: &Cmp, lc: &mut u32) -> std::io::Result<()> {
    lc.add_assign(1);
    let operation: String = format!(
        "N{2} G1 X{0:0<8.3} Y{1:0<8.3} F80\r\n", p_fin.re, p_fin.im, lc
    );
    return file.write_all(operation.as_bytes());
}

// Setup.
fn setup(mut file: &File, lc: &mut u32) -> std::io::Result<()>{
    let mut operations: String = String::from("");
    lc.add_assign(100);
    operations = operations + format!("N{0} G53\r\n", lc).as_str();
    lc.add_assign(10);
    operations = operations + format!("N{0} G71\r\n", lc).as_str();
    lc.add_assign(10);
    operations = operations + format!("N{0} G90\r\n", lc).as_str();
    lc.add_assign(10);
    operations = operations + format!("N{0} G94\r\n", lc).as_str();
    lc.add_assign(10);
    operations = operations + format!("N{0} M00\r\n", lc).as_str();
    lc.add_assign(10);
    operations = operations + format!("N{0} G54\r\n", lc).as_str();
    lc.add_assign(10);
    operations = operations + format!("N{0} M03 S2000\r\n", lc).as_str();

    return file.write_all(operations.as_bytes());
}

// End subprogram.
fn end(mut file: &File, lc: &mut u32) -> std::io::Result<()>{
    let mut operations: String = String::from("");
    *lc = (*lc / 100 + 1) * 100;
    operations = operations + format!("N{0} G0 Z30. F100\r\n", lc).as_str();
    lc.add_assign(10);
    operations = operations + format!("N{0} M05\r\n", lc).as_str();
    lc.add_assign(10);
    operations = operations + format!("N{0} M17\r\n", lc).as_str();
    return file.write_all(operations.as_bytes());
}
