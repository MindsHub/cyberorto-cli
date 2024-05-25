use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};
use reqwest::*;
use std::io::{self, stdout};

pub const BASE_URL: &str = "https://webhook.site";

fn url_join(segments: Vec<&str>) -> String {
    return segments.join("/");
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut should_quit = false;
    while !should_quit {
        terminal.draw(ui)?;
        should_quit = handle_events()?;
    }
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

#[tokio::main]
async fn send1() {
    let client = Client::new();
    let response = client
        .get(url_join(vec![
            BASE_URL,
            "1a77d6aa-450d-42bd-88cb-1e9520746786",
        ]))
        .send()
        .await
        .unwrap()
        .text()
        .await;
}
#[tokio::main]

async fn send2() {
    let client = reqwest::Client::new();
    let response = client
        .get(url_join(vec![
            BASE_URL,
            "1a77d6aa-450d-42bd-88cb-1e9520746786",
        ]))
        .send()
        .await
        .unwrap()
        .text()
        .await;
}
#[tokio::main]

async fn send3() {
    let client = reqwest::Client::new();
    let response = client
        .get(url_join(vec![
            BASE_URL,
            "1a77d6aa-450d-42bd-88cb-1e9520746786",
        ]))
        // confirm the request using send()
        .send()
        .await
        // the rest is the same!
        .unwrap()
        .text()
        .await;
}
#[tokio::main]
async fn toggle_led() {
    let client = reqwest::Client::new();
    let response = client
        .get("http://127.0.0.1:8000/toggle_led")
        // confirm the request using send()
        .send()
        .await
        // the rest is the same!
        .unwrap()
        .text()
        .await;
    println!("{:?}", response);
}

fn handle_events() -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(true);
            }
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('1') {
                println!("Faccio partire il robot...");
                println!("\nPremi <e> per continuare");
                send1();
            }
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('2') {
                println!("Fermo il robot...");
                println!("\nPremi <e> per continuare");
                send2();
            }
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('3') {
                println!("Innaffio le piante...");
                println!("\nPremi <e> per continuare");
                send3();
            }
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('4') {
                println!("Done.");
                println!("\nPremi <e> per continuare");
                toggle_led();
            }
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('e') {
                main();
            }
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('z') {
                get_request();
                println!("\nPremi <e> per continuare");
            }
        }
    }
    Ok(false)
}
#[tokio::main]
async fn get_request() {
    let response = reqwest::get("https://webhook.site/93d0c9cc-4e21-4f88-aa91-5c5caa7ec935")
        .await
        .unwrap()
        .text()
        .await;
    println!("{:?}", response);
}

const status: &str = "
        (stato robot, raspberry e altro)";

const position: &str = "
    x:
    y:               
    z:";
fn ui(frame: &mut Frame) {
    let main_layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ],
    )
    .split(frame.size());
    frame.render_widget(
        Block::new()
            .borders(Borders::TOP)
            .title("Cyberorto")
            .light_red(),
        main_layout[0],
    );

    let inner_layout = Layout::new(
        Direction::Horizontal,
        [Constraint::Percentage(55), Constraint::Percentage(45)],
    )
    .split(main_layout[1]);

    let layout_bello = Layout::new(
        Direction::Vertical,
        [Constraint::Percentage(50), Constraint::Percentage(50)],
    )
    .split(inner_layout[0]);

    let position_layout = Layout::new(
        Direction::Horizontal,
        [Constraint::Percentage(50), Constraint::Percentage(50)],
    )
    .split(layout_bello[0]);

    frame.render_widget(
        Paragraph::new(status).white().block(
            Block::default()
                .title("Stato")
                .borders(Borders::ALL)
                .light_green(),
        ),
        position_layout[0],
    );

    frame.render_widget(
        Paragraph::new(position).white().block(
            Block::default()
                .title("Posizione")
                .borders(Borders::ALL)
                .light_green(),
        ),
        position_layout[1],
    );

    frame.render_widget(
        Paragraph::new(
            "    
  1. Fai partire il robot
  2. Ferma il robot
  3. Innaffia le piante
  4. toggle_led 

  Seleziona una delle azioni (1,2,3):",
        )
        .white()
        .block(
            Block::default()
                .title("Azioni")
                .borders(Borders::ALL)
                .light_green(),
        ),
        inner_layout[1],
    );
    frame.render_widget(
        Paragraph::new(
            " 
 🌷 🌷 🌷 🌷 🌷 🌷 🌷     🌷 = Tulipano Rosa    💧 💧 💧 💧 💧 💧 💧      💧 = Innaffiata
 🌻 🌻 🌻 🌻 🌻 🌻 🌻     🌻 = Girasole         💧 💧 💧 💧 🔥 🔥 🔥      🔥 = Non innaffiata
 🥀 🥀 🌷 🥀 🌷 🌷 🥀     🥀 = Morta            🔥 🔥 💧 💧 💧 💧 🔥                                          ",
        )
        .style(Style::new().white())
        .block(
            Block::default()
                .title("Stato Piante")
                .borders(Borders::ALL)
                .green(),
        )
        .style(Style::new().white()),
        layout_bello[1],
    );
    frame.render_widget(
        Paragraph::new("Press <q> to quit   Press <e> to refresh").cyan(),
        main_layout[2],
    );
}
