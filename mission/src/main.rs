use std::io;
use std::collections::HashMap;
use std::time::Duration;
use tui::backend::{CrosstermBackend};
use tui::layout::{Constraint, Layout};
use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Paragraph};
use tui::Terminal;
use crossterm::{execute, terminal};
use crossterm::event::{self, Event, KeyCode};
use tokio::sync::mpsc;
use tokio::sync::watch;
use tokio::time::Instant;
use std::env;

#[tokio::main]
async fn main() -> Result<(), io::Error> {

    let build_env = include_str!(".././.env");

        let parsed_env: HashMap<String, String> = build_env
        .lines()
        .filter(|linha| !linha.is_empty() && !linha.starts_with('#'))
        .filter_map(|linha| {
            let mut variavel = linha.splitn(2, '=');
            Some((
                variavel.next()?.trim().to_string(),
                variavel.next()?.trim().to_string(),
            ))
        })
        .collect();

    for (k, valor) in parsed_env {
        std::env::set_var(k, valor);
    }

    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, terminal::EnterAlternateScreen)?;
    execute!(stdout, terminal::Clear(terminal::ClearType::All))?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let password = env::var("SUPER_SECRET_PASSWORD").expect("PASSWORD não encontrada");
    let flag = env::var("FLAG").expect("FLAG não encontrada");

    let (countdown_tx, countdown_rx) = watch::channel(15); 
    let mut saida = false;
    let mut input_usuario = String::new();
    let mut posicao = 0;
    let mut win = false; 
    let mut game_over = false;
    let mut win_time: Option<Instant> = None;
    let mut cursor_visivel = true;
    let mut cursor_toggle = Instant::now();
    let (tx, mut rx) = mpsc::unbounded_channel();

    tokio::spawn(async move {
        loop {
            if event::poll(Duration::from_millis(100)).unwrap() {
                if let Ok(Event::Key(key)) = event::read() {
                    if tx.send(key).is_err() {
                        break;
                    }
                }
            }
        }
    });

    tokio::spawn(async move {
        while *countdown_tx.borrow() > 0 {
            tokio::time::sleep(Duration::from_secs(1)).await;
            countdown_tx.send_modify(|countdown| *countdown -= 1);
        }
    });

    while !saida {
        if cursor_toggle.elapsed() > Duration::from_millis(500) {
            cursor_visivel = !cursor_visivel;
            cursor_toggle = Instant::now();
        }

        let countdown = *countdown_rx.borrow();
        if countdown == 0 && !game_over && !win {
            game_over = true;
            win_time = Some(Instant::now());
        }

        if win {
            if let Some(start_time) = win_time {
                if Instant::now().duration_since(start_time) > Duration::from_secs(15) {
                    saida = true;
                }
            }
        }

        terminal.draw(|f| {
            let chunks = Layout::default()
                .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
                .split(f.size());

            let countdown = if win {
                vec![Spans::from(Span::styled(
                    &flag,
                    Style::default().fg(Color::Green),
                ))]
            } else if game_over {
                format_game_over_ascii()
            } else {
                format_large_ascii_number(countdown)
            };
            let countdown_paragraph = Paragraph::new(countdown);
            f.render_widget(countdown_paragraph, chunks[0]);

            if !win && !game_over {
                let cursor = if cursor_visivel { "|" } else { " " };
                let mut input_cursor = input_usuario.clone();
                input_cursor.insert(posicao, cursor.chars().next().unwrap());
                let prompt = vec![Spans::from(vec![
                    Span::styled(">:", Style::default().fg(Color::Green)),
                    Span::raw(input_cursor),
                ])];
                let prompt_paragraph = Paragraph::new(prompt);
                f.render_widget(prompt_paragraph, chunks[1]);
            }
        })?;

        if let Ok(Some(key)) = tokio::time::timeout(Duration::from_millis(100), rx.recv()).await {
            match key.code {
                KeyCode::Char('q') => saida = true,
                KeyCode::Char(c) => {
                    input_usuario.insert(posicao, c);
                    posicao += 1; 
                }
                KeyCode::Backspace => {
                    if posicao > 0 {
                        input_usuario.remove(posicao - 1); 
                        posicao -= 1; 
                    }
                }
                KeyCode::Delete => {
                    if posicao < input_usuario.len() {
                        input_usuario.remove(posicao); 
                    }
                }
                KeyCode::Left => {
                    if posicao > 0 {
                        posicao -= 1; 
                    }
                }
                KeyCode::Right => {
                    if posicao < input_usuario.len() {
                        posicao += 1; 
                    }
                }
                KeyCode::Enter => {
                    if input_usuario == password {
                        win = true;
                        win_time = Some(Instant::now());
                    } else {
                        game_over = true;
                        win_time = Some(Instant::now());
                    }
                }
                KeyCode::Esc => saida = true,
                _ => {}
            }
        }

        if game_over {
            if let Some(start_time) = win_time {
                if Instant::now().duration_since(start_time) > Duration::from_secs(10) {
                    saida = true;
                }
            }
        }
    }

    terminal::disable_raw_mode()?;
    execute!(terminal.backend_mut(), terminal::LeaveAlternateScreen)?;
    Ok(())
}

fn format_large_ascii_number(num: i32) -> Vec<Spans<'static>> {
    let ascii_numbers = [
        r"
  █████  
 ██   ██ 
 ██   ██ 
 ██   ██ 
  █████  
        ",
        r"
    ██   
   ███   
    ██   
    ██   
  █████  
        ",
        r"
  █████  
 ██   ██ 
     ██  
   ██    
 ███████ 
        ",
        r"
  █████  
 ██   ██ 
    ███  
 ██   ██ 
  █████  
        ",
        r"
 ██   ██ 
 ██   ██ 
 ███████ 
      ██ 
      ██ 
        ",
        r"
 ███████ 
 ██      
 ██████  
      ██ 
 ██████  
        ",
        r"
  █████  
 ██      
 ██████  
 ██   ██ 
  █████  
        ",
        r"
 ███████ 
      ██ 
     ██  
    ██   
    ██   
        ",
        r"
  █████  
 ██   ██ 
  █████  
 ██   ██ 
  █████  
        ",
        r"
  █████  
 ██   ██ 
  ██████ 
      ██ 
  █████  
        ",
    ];

    let digitos: Vec<char> = format!("{:02}", num).chars().collect();
    let mut ascii_lines: Vec<String> = vec![String::new(); 6];

    for digito in digitos {
        let ascii = ascii_numbers[digito.to_digit(10).unwrap() as usize];
        for (i, line) in ascii.lines().enumerate() {
            if i < ascii_lines.len() {
                ascii_lines[i].push_str(line);
                ascii_lines[i].push(' ');
            }
        }
    }
    ascii_lines
        .into_iter()
        .map(|line| Spans::from(Span::raw(line)))
        .collect()
}

fn format_game_over_ascii() -> Vec<Spans<'static>> {
    let game_over_ascii = r"
     _.-^^---....,,--       
 _--                  --_  
<                        >)
|                         | 
 \._                   _./  
    ```--. . , ; .--'''       
          | |   |             
       .-=||  | |=-.   
       `-=#$%&%$#=-'
          | |   |             
       .-=||  | |=-.   
       `-=#$%&%$#=-'  
          | |   |             
          | ;  :|     
 _____.,-#%&$@%#&#~,._____
         GAME OVER
    ";

    let lines: Vec<Spans> = game_over_ascii
        .lines()
        .map(|line| Spans::from(Span::styled(
            line,
            Style::default().fg(Color::Red),
        )))        
        .collect();
    lines
}
