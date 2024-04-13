use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};
use reqwest::*;
use std::io::{self, stdout};

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
        .get("https://webhook.site/0b66bf02-6d1a-408f-9e90-608f3a40e7b0")
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
        .get("https://webhook.site/7ce6912c-d287-42b8-9327-b8d609efe71f")
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
        .get("https://webhook.site/f74ec52f-b5fb-4df5-aac5-5aa3359a11f3")
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
        }
    }
    Ok(false)
}

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
        [
            Constraint::Percentage(55),
            Constraint::Percentage(45),
        ],
    )
    .split(main_layout[1]);

    let layout_bello = Layout::new(
        Direction::Vertical,
        [Constraint::Percentage(50), Constraint::Percentage(50)],
    )
    .split(inner_layout[0]);

    frame.render_widget(
        Paragraph::new(
            " Stato:
     Server:   OK       Robot: In Funzione
     Internet: OK
     Client:   OK",
        )
        .white()
        .block(
            Block::default()
                .title("Informazioni")
                .borders(Borders::ALL)
                .light_green(),
        ),
        inner_layout[0],
    );
    frame.render_widget(
        Paragraph::new(
            "    
 1. Fai partire il robot
 2. Ferma il robot
 3. Innaffia le piante
 4. toggle_led 

 Seleziona una delle opzioni (1,2,3):",
        )
        .white()
        .block(
            Block::default()
                .title("Opzioni")
                .borders(Borders::ALL)
                .light_green(),
        ),
        inner_layout[1],
    );
    frame.render_widget(
        Paragraph::new(
            "
   piante e stato delle piante...",
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
