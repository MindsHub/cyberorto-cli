use block::title;
use derive_setters::Setters;
use lipsum::lipsum;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};
use reqwest::*;
use std::{clone, io::{self, stdout}, string};

#[derive(Debug, Default, Setters)]
struct Popup<'a> {
    #[setters(into)]
    title: Line<'a>,
    #[setters(into)]
    content: Text<'a>,
    border_style: Style,
    title_style: Style,
    style: Style,
}

impl Widget for Popup<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // ensure that all cells under the popup are cleared to avoid leaking content
        Clear.render(area, buf);
        let block = Block::new()
            .title(self.title)
            .title_style(self.title_style)
            .borders(Borders::ALL)
            .border_style(self.border_style);
        Paragraph::new(self.content)
            .wrap(Wrap { trim: true })
            .style(self.style)
            .block(block)
            .render(area, buf);
    }
}
pub const BASE_URL: &str = "https://webhook.site";

fn url_join(segments: Vec<&str>) -> String {
    return segments.join("/");
}
#[derive(Clone)]
enum UiState {
    Quit,
    MainScreen,
    Popup{
        title: String,
        description: String
    },
    Move{
        title: String,
        description: String
    },
    Reset_position{
        title: String,
        description: String
    },
    Retract_robot{
        title: String,
        description: String
    },
    Acqua{
        title: String,
        description: String
    },
    Luce{
        title: String,
        description: String
    },


}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal: Terminal<CrosstermBackend<io::Stdout>> = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut ui_state = UiState::MainScreen;
    let mut fn_active: i32 = 0;
    loop {
        if let Ok(Some(new_state)) = handle_events(ui_state.clone()) {
            ui_state = new_state
        }
        match ui_state.clone() {
            UiState::Quit => break,
            UiState::MainScreen => terminal.draw(ui)?,
            UiState::Popup { title, description } => terminal.draw(|frame| coso(frame, title, description))?,
            UiState::Move { title, description } => terminal.draw(|frame| coso(frame, title, description))?,
            UiState::Reset_position { title, description } => terminal.draw(|frame| coso(frame, title, description))?,
            UiState::Retract_robot { title, description } => terminal.draw(|frame| coso(frame, title, description))?,
            UiState::Acqua { title, description } => terminal.draw(|frame| coso(frame, title, description))?,
            UiState::Luce { title, description } => terminal.draw(|frame| coso(frame, title, description))?,
            
        };
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

fn handle_events(state:  UiState) -> io::Result<Option<UiState>> {
    if event::poll(std::time::Duration::from_millis(10))? {
        match state{
            UiState::Quit => {

            }
            UiState::MainScreen => {
                if let Event::Key(key) = event::read()? {
                    if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                        return Ok(Some(UiState::Quit));
                    }
                    if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('1') {
                        return Ok(Some(UiState::Popup { title: ("Muovi".to_string()), description: ("Inserisci le coordinate dove vuoi spostare il robot:
                         x:
                         y:
                         z:
                         Premere <e> per confermare...
                        ".to_string()) }));
                    }

                }
            },
            UiState::Popup { title, description } => {
                if let Event::Key(key) = event::read()? {
                    if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('e') {
                        send1();
                        return Ok(Some(UiState::MainScreen));
                    }
                }

            },
            UiState::Move { title, description } => {
                if let Event::Key(key) = event::read()? {
                    if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('e') {
                        send1();
                        return Ok(Some(UiState::MainScreen));
                    }
                }
            }
            ,
            UiState::Reset_position { title, description } => {
                if let Event::Key(key) = event::read()? {
                    if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('e') {
                        send1();
                        return Ok(Some(UiState::MainScreen));
                    }
                }
            },
            UiState::Retract_robot { title, description } => {
                if let Event::Key(key) = event::read()? {
                    if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('e') {
                        send1();
                        return Ok(Some(UiState::MainScreen));
                    }
                }
            },
            UiState::Acqua { title, description } => {
                if let Event::Key(key) = event::read()? {
                    if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('e') {
                        send1();
                        return Ok(Some(UiState::MainScreen));
                    }
                }
            },
            UiState::Luce { title, description } => {
                if let Event::Key(key) = event::read()? {
                    if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('e') {
                        send1();
                        return Ok(Some(UiState::MainScreen));
                    }
                }
            },
        }
        if let Event::Key(key) = event::read()? {;
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(Some(UiState::Quit));
            }
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('1') {
                return Ok(Some(UiState::Move { title: ("Muovi".to_string()), description: ("Inserisci le coordinate dove vuoi spostare il robot:
                 x:
                 y:
                 z:
                 Premere <e> per confermare...
                ".to_string()) }));
                send1();
            }
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('2') {
                return Ok(Some(UiState::Reset_position { title: ("Muovi".to_string()), description: ("Sto resettando il robot alla posizione 0...
 


                Premere <e> per continuare
               ".to_string()) }));
               send2();
            }
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('3') {
                
                return Ok(Some(UiState::Retract_robot { title: ("Muovi".to_string()), description: ("Sto retractando il braccio del robot...


                Premere <e> per continuare
               ".to_string()) }));
               
               send3();
            }
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('4') {
                return Ok(Some(UiState::Acqua { title: ("Innaffia".to_string()), description: ("Per quanto tempo vuoi innaffiare?
                                    (input)


                Premere <e> per confermare
               ".to_string()) }));
            }
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('5') {
                
                return Ok(Some(UiState::Luce { title: ("Illumina".to_string()), description: ("Per quanto tempo vuoi illuminare?
                                    (input)


                Premere <e> per confermare
               ".to_string()) }));
            }
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('e') {
                return Ok(Some(UiState::MainScreen));
            }
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('z') {
                get_request();
                println!("\nPremi <e> per continuare");
            }
        }
    }
    Ok(None)
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
        Arduino: OK
        Raspberry:OK
        Manutenzione richiesta: NO";
const popup_name: &str = "";
const popup_content: &str = "";
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
                .title("Posizione attuale")
                .borders(Borders::ALL)
                .light_green(),
        ),
        position_layout[1],
    );

    frame.render_widget(
        Paragraph::new(
            "    
  1. Muovi il robot in una coordinata precisa
  2. Muovi il robot nella posizione iniziale
  3. Retrai il braccio del robot
  4. Innaffia (durata)
  5. Illumina (durata)

  Seleziona una delle azioni (1,2,3,4,5):",
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
fn coso (frame: &mut Frame, title: String, description: String) {
    let area = frame.size();
    let popup_area: Rect = Rect {
    x: area.width / 4,
    y: area.height / 3,
    width: area.width / 2,
    height: area.height / 2,
    };
    let bad_popup = Paragraph::new("Hello world!")
    .wrap(Wrap { trim: true })
    .style(Style::new().yellow())
    .block(
        Block::new()
            .title("Without Clear")
            .title_style(Style::new().white().bold())
            .borders(Borders::ALL)
            .border_style(Style::new().red()),
    );
    let popup = Popup::default()
    .content(description)
    .style(Style::new().white())
    .title(title)
    .title_style(Style::new().green().bold())
    .border_style(Style::new().green());
    frame.render_widget(popup, popup_area);
}
