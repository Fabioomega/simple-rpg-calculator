# Simple Homebrew RPG Calculator
A simple calculator used in my homebrew rpg made to facilitate some of the more annoying calculations.
Written in rust.

## Compiling

Just do a `cargo build` inside the project and you should be fine. Remember you need to have cargo installed in your path. For more information on how to install rust see: [Rust language and install guide](https://www.rust-lang.org "Rust language and install guide").

## Requirements

You need a file in the same directory of the program that contain all the magics you have access to. It should be named `init.rpg`.

## Init Schema

The `init.rpg` file have the following schema:
```
register <magic_name>
{
    rank <unsigned int>
    type <ORDER | CHAOS>
    always_def <bool>
    table_addon <int>
    race_mult <float>
}
```
- `magic_name` : the name of your magic.
- `rank` : is your current magic rank. It should be between 0 and 5.
- `type` : should be your magic origin.
- `always_def` : whether to always treat that magic as defensive.
- `table_addon` : the value to add to the default accuracy when plotting a table.
- `race_mult` : the multiplier that your race may have with this magic.

Example:

```
register fire
{
    rank 0
    type ORDER
    always_def false
    table_addon 0
    race_mult 1.2
}
```

## Using the calculator

### Magical Functions

- To see the basic usage do: `<magic_name>()`.
- To see the plotted table usage do: `t_<magic_name>()`.
- To see the defensive mode usage do: `def_<magic_name>()`. Only available if `always_def` is false.
- To see the plotted table for the defensive mode usage do: `t_def_<magic_name>()`. Only available if `always_def` is false.

Also, if your magic is `always_def` and you want to see the attack the attack usage you do as follows:
- To see the attack usage do: `at_<magic_name>()`.
- To see the attack plotted table usage do: `t_at_<magic_name>()`.

### Common Calculator Things

As a calculator the program can also compute any kind of expressions that a calculator is able to handle.
You can also initialize variables _python-like_ like so: `a = 12`. That means that a variable called `a` was initialized withe the value `12`. That variable may be used in any function. So given a magic name fire, you may do `fire(a, a)` which will output the fire magic with the accuracy of 12 and 12 mana spent on it.
