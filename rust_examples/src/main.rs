mod error;

use error::{Error, Result};
use z3::ast::{Ast, Int};
use z3::{ast, Config, Context, Optimize, SatResult, Solver};

fn main() -> Result<()> {
    println!("Hello, world!");
    Ok(())
}

#[test]
fn test_shirt_tie() -> Result<()> {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);
    let shirt = ast::Bool::new_const(&ctx, "shirt");
    let tie = ast::Bool::new_const(&ctx, "tie");
    solver.assert(&tie.or(&[&shirt]));
    solver.assert(&tie.not().or(&[&shirt]));
    solver.assert(&tie.not().or(&[&shirt.not()]));
    assert_eq!(solver.check(), SatResult::Sat);
    let model = solver.get_model();
    assert_eq!(model.eval(&tie).unwrap().as_bool(), Some(false));
    assert_eq!(model.eval(&shirt).unwrap().as_bool(), Some(true));
    println!("test_shirt_tie passed.");
    Ok(())
}

#[test]
fn test_xkcd_solver() -> Result<()> {
    // example problem:
    // https://xkcd.com/287/

    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    let mixed_fruit = ast::Int::new_const(&ctx, "mixed fruit");
    let mixed_fruit_price = ast::Int::from_u64(&ctx, 215);
    let french_fries = ast::Int::new_const(&ctx, "french fries");
    let french_fries_price = ast::Int::from_u64(&ctx, 275);
    let side_salad = ast::Int::new_const(&ctx, "side salad");
    let side_salad_price = ast::Int::from_u64(&ctx, 335);
    let hot_wings = ast::Int::new_const(&ctx, "hot wings");
    let hot_wings_price = ast::Int::from_u64(&ctx, 355);
    let mozzarella_sticks = ast::Int::new_const(&ctx, "mozarella sticks");
    let mozzarella_sticks_price = ast::Int::from_u64(&ctx, 420);
    let sampler_plate = ast::Int::new_const(&ctx, "sampler plate");
    let sampler_placte_price = ast::Int::from_u64(&ctx, 580);

    let zero = ast::Int::from_u64(&ctx, 0);

    solver.assert(&mixed_fruit.ge(&zero));
    solver.assert(&french_fries.ge(&zero));
    solver.assert(&side_salad.ge(&zero));
    solver.assert(&hot_wings.ge(&zero));
    solver.assert(&mozzarella_sticks.ge(&zero));
    solver.assert(&sampler_plate.ge(&zero));

    let fruit_mul = mixed_fruit.mul(&[&mixed_fruit_price]);
    let fries_mul = french_fries.mul(&[&french_fries_price]);
    let salad_mul = side_salad.mul(&[&side_salad_price]);
    let wings_mul = hot_wings.mul(&[&hot_wings_price]);
    let mozzarella_mul = mozzarella_sticks.mul(&[&mozzarella_sticks_price]);
    let sampler_mul = sampler_plate.mul(&[&sampler_placte_price]);
    solver.assert(
        &fruit_mul
            .add(&[
                &fries_mul,
                &salad_mul,
                &wings_mul,
                &mozzarella_mul,
                &sampler_mul,
            ])
            ._eq(&ast::Int::from_u64(&ctx, 1505)),
    );
    println!();
    let mut answers = Vec::<Vec<u64>>::new();

    // Since there is more than one answer, we iterate over our model:
    while solver.check() == SatResult::Sat {
        let model = solver.get_model();
        let mixed_fruit_res = model.eval(&mixed_fruit).unwrap();
        let french_fries_res = model.eval(&french_fries).unwrap();
        let side_salad_res = model.eval(&side_salad).unwrap();
        let hot_wings_res = model.eval(&hot_wings).unwrap();
        let mozzarella_sticks_res = model.eval(&mozzarella_sticks).unwrap();
        let sampler_plate_res = model.eval(&sampler_plate).unwrap();

        println!("mixed_fruit_res: {}", mixed_fruit_res);
        println!("french_fries_res: {}", french_fries_res);
        println!("side_salad_res: {}", side_salad_res);
        println!("hot_wings_res: {}", hot_wings_res);
        println!("mozzarella_sticks: {}", mozzarella_sticks_res);
        println!("sampler_plate_res: {}", sampler_plate_res);
        println!();

        answers.push(vec![
            mixed_fruit_res.as_u64().unwrap(),
            french_fries_res.as_u64().unwrap(),
            side_salad_res.as_u64().unwrap(),
            hot_wings_res.as_u64().unwrap(),
            mozzarella_sticks_res.as_u64().unwrap(),
            sampler_plate_res.as_u64().unwrap(),
        ]);

        // add a new condition so that the next .check() iteration skips the current solution:

        solver.assert(&mixed_fruit._eq(&mixed_fruit_res).not().or(&[
            &french_fries._eq(&french_fries_res).not(),
            &side_salad._eq(&side_salad_res).not(),
            &hot_wings._eq(&hot_wings_res).not(),
            &mozzarella_sticks._eq(&mozzarella_sticks_res).not(),
            &sampler_plate._eq(&sampler_plate_res).not(),
        ]));
    }
    assert_eq!(
        answers,
        vec![vec![1, 0, 0, 2, 0, 1], vec![7, 0, 0, 0, 0, 0]]
    );
    println!("test_xkcd passed.");
    Ok(())
}

#[test]
fn test_wood_workshop_solver() -> Result<()> {
    // Problem is on p.31 here:
    // https://yurichev.com/writings/SAT_SMT_by_example.pdf

    // Linear Programming.

    // you have a 6"x13" plywood work pieces
    // you need 800 rectangles that are 4"x5" (Output A)
    // you need 400 rectangles that are 2"x3" (Output B)

    // To cut a piece as A/B rectangles, you can cut a 6*13 workpiece in 4 ways. Or, to put it in
    // another way, you can placeA/B rectangles on 6*13 rectangle in 4 ways:

    // - Cut A (Output A: 3, Output B: 1)
    // - Cut B (Output A: 2, Output B: 6)
    // - Cut C (Output A: 1, Output B: 9)
    // - Cut D (Output A: 0, Output B: 13)

    // Which cuts are most efficient? You want to consume as little workpieces as possible.

    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let optimize = Optimize::new(&ctx);

    let workpieces_total = ast::Int::new_const(&ctx, "workpieces total");
    let a_cuts = ast::Int::new_const(&ctx, "a cuts");
    let b_cuts = ast::Int::new_const(&ctx, "b cuts");
    let c_cuts = ast::Int::new_const(&ctx, "c cuts");
    let d_cuts = ast::Int::new_const(&ctx, "d cuts");

    let output_a = ast::Int::new_const(&ctx, "output a");
    let output_b = ast::Int::new_const(&ctx, "output b");

    let zero = ast::Int::from_u64(&ctx, 0);
    optimize.assert(&a_cuts.ge(&zero));
    optimize.assert(&b_cuts.ge(&zero));
    optimize.assert(&c_cuts.ge(&zero));
    optimize.assert(&d_cuts.ge(&zero));

    optimize.assert(
        &a_cuts
            .add(&[&b_cuts, &c_cuts, &d_cuts])
            ._eq(&workpieces_total),
    );

    // set the requirement that we must have 800 output_a's
    optimize.assert(
        &a_cuts
            .mul(&[&ast::Int::from_u64(&ctx, 3)])
            .add(&[
                &b_cuts.mul(&[&ast::Int::from_u64(&ctx, 2)]),
                &c_cuts.mul(&[&ast::Int::from_u64(&ctx, 1)]),
                &d_cuts.mul(&[&ast::Int::from_u64(&ctx, 0)]), // remove this?
            ])
            ._eq(&ast::Int::from_u64(&ctx, 800)),
    );

    // set the requirement that we must have 400 output_a's
    optimize.assert(
        &a_cuts
            .mul(&[&ast::Int::from_u64(&ctx, 1)])
            .add(&[
                &b_cuts.mul(&[&ast::Int::from_u64(&ctx, 6)]),
                &c_cuts.mul(&[&ast::Int::from_u64(&ctx, 9)]),
                &d_cuts.mul(&[&ast::Int::from_u64(&ctx, 13)]),
            ])
            ._eq(&ast::Int::from_u64(&ctx, 400)),
    );

    optimize.minimize(&workpieces_total);

    assert_eq!(optimize.check(&[]), SatResult::Sat);
    let model = optimize.get_model();
    let a_cuts_res = model.eval(&a_cuts).unwrap().as_u64().unwrap();
    let b_cuts_res = model.eval(&b_cuts).unwrap().as_u64().unwrap();
    let c_cuts_res = model.eval(&c_cuts).unwrap().as_u64().unwrap();
    let d_cuts_res = model.eval(&d_cuts).unwrap().as_u64().unwrap();
    let workpieces_total_res = model.eval(&workpieces_total).unwrap().as_u64().unwrap();

    println!(
        "a_cuts: {}, b_cuts: {}, c_cuts: {}, d_cuts: {}, workpieces_total: {}",
        a_cuts_res, b_cuts_res, c_cuts_res, d_cuts_res, workpieces_total_res
    );
    assert_eq!(a_cuts_res, 250);
    assert_eq!(b_cuts_res, 25);
    assert_eq!(c_cuts_res, 0);
    assert_eq!(d_cuts_res, 0);
    assert_eq!(workpieces_total_res, 275);

    println!("test_wood_workshop passed.");
    Ok(())
}
