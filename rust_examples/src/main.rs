mod error;

use error::{Error, Result};
use z3::ast::{Ast, Int};
use z3::{ast, Config, Context, SatResult, Solver, Optimize};

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

    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let optimize = Optimize::new(&ctx);

    println!("test_wood_workshop passed.");
    Ok(())
}
