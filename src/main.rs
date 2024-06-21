use crossterm::{
    cursor, execute, queue,
    style::{self, Print, PrintStyledContent, Stylize},
    terminal, ExecutableCommand, QueueableCommand,
};
use evalexpr::*;
use handle_file::MagicRank;
use handle_file::MagicType;
use prettytable::row;
use std::string::String;
use std::{
    fmt::Debug,
    io::{stdout, Write},
};

pub mod handle_file;
use handle_file::process_file_to_magic;
use handle_file::Magic;

#[inline]
fn get_multiplier(rank: MagicRank, typ: MagicType) -> f64 {
    match rank {
        MagicRank::Common => match typ {
            MagicType::ORDER => 4.0,
            MagicType::CHAOS => 5.5,
        },
        MagicRank::Uncommon => match typ {
            MagicType::ORDER => 6.0,
            MagicType::CHAOS => 7.5,
        },
        MagicRank::Epic => match typ {
            MagicType::ORDER => 9.0,
            MagicType::CHAOS => 10.5,
        },
        MagicRank::Legendary => match typ {
            MagicType::ORDER => 13.0,
            MagicType::CHAOS => 14.5,
        },
        MagicRank::Mythic => match typ {
            MagicType::ORDER => 18.0,
            MagicType::CHAOS => 19.5,
        },
        MagicRank::Divine => match typ {
            MagicType::ORDER => 24.0,
            MagicType::CHAOS => 25.5,
        },
    }
}

#[inline]
fn calculate_total_damage(mana: i64, rank: MagicRank, typ: MagicType) -> f64 {
    (mana as f64) * get_multiplier(rank, typ).floor()
}

#[inline]
fn calculate_accuracy(accr: i64, typ: MagicType) -> f64 {
    let t: f64 = accr as f64;
    match typ {
        MagicType::ORDER => 0.5 + t * 0.025,
        MagicType::CHAOS => {
            if accr % 2 == 0 {
                0.5 + t * 0.025
            } else {
                0.5 + t * 0.18 / 8.0
            }
        }
    }
}

#[inline]
fn calculate_effective_damage(
    accr: i64,
    mana: i64,
    rank: MagicRank,
    typ: MagicType,
    mul: f64,
) -> i64 {
    (calculate_total_damage(mana, rank, typ) * calculate_accuracy(accr, typ) * mul).floor() as i64
}

#[inline]
fn calculate_effective_damage_f64(
    accr: i64,
    mana: i64,
    rank: MagicRank,
    typ: MagicType,
    mul: f64,
) -> f64 {
    (calculate_total_damage(mana, rank, typ) * calculate_accuracy(accr, typ) * mul).floor()
}

#[inline]
fn calculate_defense_life(accr: i64, mana: i64, rank: MagicRank, typ: MagicType, mul: f64) -> i64 {
    let r = calculate_effective_damage_f64(accr, mana, rank, typ, mul);
    match typ {
        MagicType::ORDER => (1.3 * r) as i64,
        MagicType::CHAOS => ((1.3 * r) - r.ln()) as i64,
    }
}

#[inline]
fn default_help_message(
    magic_name: &String,
    always_defensive: bool,
    should_be_defensive: bool,
) -> String {
    if !always_defensive {
        if !should_be_defensive {
            format!("Use {:}(<accuracy>, <mana>)", magic_name)
        } else {
            format!("Use def_{:}(<accuracy>, <mana>)", magic_name)
        }
    } else {
        if !should_be_defensive {
            format!("Use {:}(<accuracy>, <mana>)", magic_name)
        } else {
            format!("Use at_{:}(<accuracy>, <mana>)", magic_name)
        }
    }
}

fn table_help_message(
    magic_name: &String,
    always_defensive: bool,
    should_be_defensive: bool,
) -> String {
    if !always_defensive {
        if !should_be_defensive {
            format!(
                "Use t_{:}(<start>, <end>, <?step>, <?accuracy>)",
                magic_name
            )
        } else {
            format!(
                "Use t_def_{:}(<start>, <end>, <?step>, <?accuracy>)",
                magic_name
            )
        }
    } else {
        if !should_be_defensive {
            format!(
                "Use t_{:}(<start>, <end>, <?step>, <?accuracy>)",
                magic_name
            )
        } else {
            format!(
                "Use t_at_{:}(<start>, <end>, <?step>, <?accuracy>)",
                magic_name
            )
        }
    }
}

fn generate_default_function(should_be_defensive: bool, mag: &Magic) -> Function {
    let rank = mag.rank.clone();
    let typ = mag.typ.clone();
    let mult = mag.race_mult;
    let name = mag.name.clone();
    let always_defensive = mag.always_def;
    Function::new(move |arguments: &Value| {
        let args = if let Ok(result) = arguments.as_tuple() {
            result
        } else {
            return Err(EvalexprError::CustomMessage(default_help_message(
                &name,
                always_defensive,
                should_be_defensive,
            )));
        };

        if args.len() != 2 {
            return Err(EvalexprError::CustomMessage(default_help_message(
                &name,
                always_defensive,
                should_be_defensive,
            )));
        };

        if let (Value::Int(accuracy), Value::Int(mana)) = (&args[0], &args[1]) {
            if !should_be_defensive {
                Ok(Value::Int(calculate_effective_damage(
                    *accuracy, *mana, rank, typ, mult,
                )))
            } else {
                Ok(Value::Int(calculate_defense_life(
                    *accuracy, *mana, rank, typ, mult,
                )))
            }
        } else {
            Err(EvalexprError::CustomMessage(default_help_message(
                &name,
                always_defensive,
                should_be_defensive,
            )))
        }
    })
}

fn generate_default_table_function(should_be_defensive: bool, mag: &Magic) -> Function {
    let rank = mag.rank.clone();
    let typ = mag.typ.clone();
    let mult = mag.race_mult;
    let table_addon = mag.table_addon;
    let name = mag.name.clone();
    let always_defensive = mag.always_def;
    Function::new(move |arguments| {
        let args = if let Ok(result) = arguments.as_tuple() {
            result
        } else {
            return Err(EvalexprError::CustomMessage(table_help_message(
                &name,
                always_defensive,
                should_be_defensive,
            )));
        };

        if args.len() != 2 && args.len() != 3 && args.len() != 4 {
            return Err(EvalexprError::CustomMessage(table_help_message(
                &name,
                always_defensive,
                should_be_defensive,
            )));
        };

        if args.len() == 2 {
            if let (Value::Int(start), Value::Int(end)) = (&args[0], &args[1]) {
                let mut table = prettytable::Table::new();

                table.add_row(row!["Mana", "Damage", "Accuracy"]);

                if !should_be_defensive {
                    for i in *start..=*end {
                        table.add_row(row![
                            i,
                            calculate_effective_damage(10 + table_addon, i, rank, typ, mult),
                            10 + table_addon
                        ]);
                    }
                } else {
                    for i in *start..=*end {
                        table.add_row(row![
                            i,
                            calculate_defense_life(10 + table_addon, i, rank, typ, mult),
                            10 + table_addon
                        ]);
                    }
                }

                table.printstd();

                Ok(Value::Empty)
            } else {
                Err(EvalexprError::CustomMessage(table_help_message(
                    &name,
                    always_defensive,
                    should_be_defensive,
                )))
            }
        } else {
            let accuracy: i64 = if args.len() == 4 {
                if let (Value::Int(acc)) = &args[2] {
                    *acc
                } else {
                    return Err(EvalexprError::CustomMessage(table_help_message(
                        &name,
                        always_defensive,
                        should_be_defensive,
                    )));
                }
            } else {
                10 + table_addon
            };

            if let (Value::Int(start), Value::Int(end), Value::Int(step)) =
                (&args[0], &args[1], &args[2])
            {
                let mut table = prettytable::Table::new();

                table.add_row(row!["Mana", "Damage", "Accuracy"]);

                if !should_be_defensive {
                    for i in (*start..=*end).step_by(*step as usize) {
                        table.add_row(row![
                            i,
                            calculate_effective_damage(accuracy, i, rank, typ, mult),
                            accuracy
                        ]);
                    }
                } else {
                    for i in (*start..=*end).step_by(*step as usize) {
                        table.add_row(row![
                            i,
                            calculate_defense_life(accuracy, i, rank, typ, mult),
                            accuracy
                        ]);
                    }
                }

                table.printstd();

                Ok(Value::Empty)
            } else {
                Err(EvalexprError::CustomMessage(table_help_message(
                    &name,
                    always_defensive,
                    should_be_defensive,
                )))
            }
        }
    })
}

fn main() {
    let blue = style::Color::Rgb {
        r: 115,
        g: 170,
        b: 218,
    };
    let orange = style::Color::Rgb {
        r: 255,
        g: 188,
        b: 65,
    };

    let mut inp: String = String::new();
    let mut context = HashMapContext::new();
    let mut magics: Vec<Magic> = match process_file_to_magic("init.rpg") {
        Ok(r) => r,
        Err(e) => {
            eprintln!("ERROR: {}", e);
            std::io::stdin().read_line(&mut inp).unwrap();
            return;
        }
    };

    let mut stdout = stdout();

    for m in magics {
        let mag = m;
        let new_name = mag.name.clone();
        if !mag.always_def {
            context
                .set_function(mag.name.clone(), generate_default_function(false, &mag))
                .expect("Function should not have any problems loading!");

            context
                .set_function(
                    "def_".to_string() + &new_name,
                    generate_default_function(true, &mag),
                )
                .expect("Function should not have any problems loading!");

            context
                .set_function(
                    "t_".to_string() + &new_name,
                    generate_default_table_function(false, &mag),
                )
                .expect("Function should not have any problems loading!");

            context
                .set_function(
                    "t_def_".to_string() + &new_name,
                    generate_default_table_function(true, &mag),
                )
                .expect("Function should not have any problems loading!");
        } else {
            context
                .set_function(
                    mag.name.clone(),
                    generate_default_table_function(true, &mag),
                )
                .expect("Function should not have any problems loading!");
            context
                .set_function(
                    "at_".to_string() + &new_name,
                    generate_default_table_function(false, &mag),
                )
                .expect("Function should not have any problems loading!");

            context
                .set_function(
                    "t_".to_string() + &new_name,
                    generate_default_table_function(true, &mag),
                )
                .expect("Function should not have any problems loading!");

            context
                .set_function(
                    "t_at_".to_string() + &new_name,
                    generate_default_table_function(false, &mag),
                )
                .expect("Function should not have any problems loading!");
        }
    }

    loop {
        queue!(
            stdout,
            PrintStyledContent(">>> ".with(style::Color::Rgb {
                r: 137,
                g: 221,
                b: 255
            }))
        )
        .unwrap();
        execute!(stdout, crossterm::style::SetForegroundColor(orange)).unwrap();
        std::io::stdin().read_line(&mut inp).unwrap();
        let result = eval_with_context_mut(inp.trim(), &mut context);
        match result {
            Ok(out) => {
                if !out.is_empty() {
                    queue!(
                        stdout,
                        PrintStyledContent("····→ ".with(blue)),
                        PrintStyledContent(out.to_string().with(orange)),
                        Print("\n")
                    )
                    .unwrap();
                }
            }
            Err(out) => {
                queue!(
                    stdout,
                    PrintStyledContent("····→ : ".red()),
                    PrintStyledContent(out.to_string().red()),
                    Print("\n")
                )
                .unwrap();
            }
        };
        inp.clear();
    }
}
