use std::{io, sync::atomic::AtomicU16, time::Duration};

use bevy_ecs::prelude::*;
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{
        DisableFocusChange, DisableMouseCapture, EnableFocusChange, EnableMouseCapture, Event,
        KeyCode, KeyModifiers, poll, read,
    },
    execute,
    style::Print,
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode},
};
use lazy_static::lazy_static;

use crate::{
    RUNNING,
    common::{Input, LdSt, View},
    scoundrel::{Action, Board, DrawDeck, Health, Selection, WeaponSlot},
};

lazy_static! {
    static ref STARTX: u16 = 4;
    static ref STARTY: u16 = 2;
    static ref X: AtomicU16 = AtomicU16::new(*STARTX);
    static ref Y: AtomicU16 = AtomicU16::new(*STARTY);
}

pub struct Tui;

fn reset_position() {
    X.st(*STARTX);
    Y.st(*STARTY);
}

fn print_health(health: &Health) {
    execute!(
        io::stdout(),
        MoveTo(X.ld(), Y.inc()),
        Print(format!("{} health", health.0)),
    )
    .expect("display health failed");
}

fn print_board(board: &Board, selection: &Selection) {
    let fmt_slot = |i: usize| {
        format!(
            "{} {}",
            match board.0[i] {
                Some(c) => c.emojify(),
                None => String::from("    "),
            },
            if selection.0 == i {
                "selected"
            } else {
                "          "
            }
        )
    };

    execute!(
        io::stdout(),
        MoveTo(X.ld(), Y.inc()),
        Print(fmt_slot(0)),
        MoveTo(X.ld(), Y.inc()),
        Print(fmt_slot(1)),
        MoveTo(X.ld(), Y.inc()),
        Print(fmt_slot(2)),
        MoveTo(X.ld(), Y.inc()),
        Print(fmt_slot(3))
    )
    .expect("display fail");
}

fn print_weapon(weapon: &WeaponSlot) {
    let weapon_string = match weapon {
        WeaponSlot::Empty => String::new(),
        WeaponSlot::Weapon { weapon, enemies } if enemies.is_empty() => {
            weapon.emojify() + "                  "
        }
        WeaponSlot::Weapon { weapon, enemies } => {
            format!(
                "{} stabbing {}",
                weapon.emojify(),
                enemies.peek().unwrap().emojify()
            )
        }
    };

    execute!(io::stdout(), MoveTo(X.ld(), Y.inc()), Print(weapon_string))
        .expect("display weapon failed");
}

fn print_draw(draw: &DrawDeck) {
    execute!(
        io::stdout(),
        MoveTo(X.ld(), Y.inc()),
        Print(format!("{} cards left", draw.0.count()))
    )
    .expect("display draw failed");
}

pub fn print_log(msg: &'static str) {
    execute!(
        io::stdout(),
        MoveTo(20, 20),
        // Print(format!("{} cards left", draw.0.count()))
        Print(msg.to_string() + "                    ")
    )
    .expect("display log failed");
}

impl View for Tui {
    fn init_display() {
        execute!(
            std::io::stdout(),
            EnableFocusChange,
            EnableMouseCapture,
            Clear(ClearType::All),
            Hide,
        )
        .unwrap();
        enable_raw_mode().expect("raw mode failed");
    }

    fn deinit_display() {
        execute!(
            std::io::stdout(),
            DisableFocusChange,
            DisableMouseCapture,
            Show,
        )
        .unwrap();
        disable_raw_mode().expect("disable raw mode failed");
    }

    fn display(
        healths: Query<&Health>,
        boards: Query<&Board>,
        weapons: Query<&WeaponSlot>,
        draws: Query<&DrawDeck>,
        selections: Query<&Selection>,
    ) {
        let health = healths.get_single().unwrap();
        let board = boards.get_single().unwrap();
        let weapon = weapons.get_single().unwrap();
        let draw = draws.get_single().unwrap();
        let selection = selections.get_single().unwrap();

        reset_position();
        print_health(health);
        print_board(board, selection);
        print_weapon(weapon);
        print_draw(draw);
    }
}

impl Input for Tui {
    fn process_input(mut actions: Query<&mut Action>) {
        let mut action = actions.single_mut();

        if poll(Duration::from_millis(100)).expect("poll failed") {
            match read().unwrap() {
                Event::Key(key_event) => match key_event.code {
                    KeyCode::Enter => {
                        *action = Action::EndTurn;
                        return;
                    }
                    KeyCode::Char(' ') => {
                        *action = Action::Select;
                        return;
                    }
                    KeyCode::Backspace => {
                        *action = Action::SkipRoom;
                        return;
                    }
                    KeyCode::Char('p') => {
                        *RUNNING.lock().unwrap() = false;
                    }
                    KeyCode::Char('c') => {
                        if key_event.modifiers.contains(KeyModifiers::CONTROL) {
                            let mut run_ref = RUNNING.lock().expect("Couldn't acquire RUNNING");
                            *run_ref = false;
                            return;
                        }
                    }
                    KeyCode::Left | KeyCode::Up | KeyCode::Char('a') | KeyCode::Char('w') => {
                        *action = Action::Left;
                        return;
                    }
                    KeyCode::Right | KeyCode::Down | KeyCode::Char('d') | KeyCode::Char('s') => {
                        *action = Action::Right;
                        return;
                    }
                    // KeyCode::Char(n) if ['1', '2', '3', '4'].contains(&n) => {
                    //     *action = Action::Slot(n as usize - '0' as usize);
                    //     return;
                    // }
                    KeyCode::Char(n @ '1'..='4') => {
                        *action = Action::Slot(n.to_digit(10).unwrap() as usize);
                        return;
                    }
                    _ => {}
                },
                // crossterm::event::Event::FocusGained => todo!(),
                // crossterm::event::Event::FocusLost => todo!(),
                // crossterm::event::Event::Mouse(mouse_event) => todo!(),
                // crossterm::event::Event::Paste(_) => todo!(),
                // crossterm::event::Event::Resize(_, _) => todo!(),
                _ => {}
            };
        }
        *action = Action::Idle
    }
}
