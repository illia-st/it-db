use clap::{Command, Arg, ArgAction};
use ratatui::{
    prelude::{CrosstermBackend, Terminal},
    widgets::Paragraph,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)?;

    let mut command = 
        clap::Command::new("database")
            .author("Tretiakov Yehor, egorken3v@gmail.com")
            .version("0.0.1")
            .about("Some cool info!")
            .subcommands([
                clap::Command::new("create")
                    .args([
                        Arg::new("database")
                            .short('d')
                            .conflicts_with("table")
                            .required_unless_present("table")
                            .action(ArgAction::SetTrue),
                        Arg::new("table")
                            .short('t')
                            .conflicts_with("database")
                            .required_unless_present("database")
                            .action(ArgAction::SetTrue),

                        Arg::new("name")
                            .short('n')
                            .required(true)
                            .action(ArgAction::Set),
                        
                        Arg::new("database_path")
                            .short('p')
                            .conflicts_with("table")
                            .required_unless_present("table")
                            .action(ArgAction::Set),

                        Arg::new("table_column_names")
                            .short('c')
                            .conflicts_with("database")
                            .required_unless_present("database")
                            .action(ArgAction::Set),
                        Arg::new("table_types")
                            .short('v')
                            .conflicts_with("database")
                            .required_unless_present("database")
                            .action(ArgAction::Set),
                    ]),

                clap::Command::new("delete")
                    .args([
                        Arg::new("database")
                            .short('d')
                            .conflicts_with("table")
                            .required_unless_present("table")
                            .action(ArgAction::SetTrue),
                        Arg::new("table")
                            .short('t')
                            .conflicts_with("database")
                            .required_unless_present("database")
                            .action(ArgAction::SetTrue),

                        Arg::new("table_name")
                            .short('n')
                            .conflicts_with("database")
                            .required_unless_present("database")
                            .action(ArgAction::Set),
                        
                        Arg::new("database_path")
                            .short('p')
                            .conflicts_with("table")
                            .required_unless_present("table")
                            .action(ArgAction::Set),
                    ]),

                clap::Command::new("open")
                    .args([
                        Arg::new("database_path")
                            .short('p')
                            .required(true)
                            .action(ArgAction::Set)
                    ]),

                clap::Command::new("close")
                    .args([
                        Arg::new("save")
                            .short('s')
                            .required(false)
                            .action(ArgAction::SetTrue)
                    ]),

                clap::Command::new("add")
                    .args([
                        Arg::new("table_name")
                            .short('n')
                            .required(true)
                            .action(ArgAction::Set),
                        Arg::new("row_value")
                            .short('r')
                            .required(true)
                            .action(ArgAction::Set)
                    ]),
                
                clap::Command::new("remove")
                    .args([
                        Arg::new("table_name")
                            .short('n')
                            .required(true)
                            .action(ArgAction::Set),
                    ]),

                clap::Command::new("rename")
                    .args([
                        Arg::new("table_name")
                            .short('n')
                            .required(true)
                            .action(ArgAction::Set),
                        Arg::new("table_column_names")
                            .short('c')
                            .required(true)
                            .action(ArgAction::Set),
                    ]),

                clap::Command::new("join")
                    .args([
                        Arg::new("left_table_name")
                            .short('l')
                            .required(true)
                            .action(ArgAction::Set),
                        Arg::new("right_table_name")
                            .short('r')
                            .required(true)
                            .action(ArgAction::Set),
                    ]),
            ]);

    let args = vec!["database", "create", "-t"];
    assert!(command.try_get_matches_from_mut(args).is_err());
    let args = vec!["database", "create", "-t", "-n", "\"\"", "-c", "\"\"", "-v", "\"\""];
    assert!(command.try_get_matches_from_mut(args).is_ok());
    let args = vec!["database", "create", "-d", "-n", "\"\"", "-p", "\"\""];
    assert!(command.try_get_matches_from_mut(args).is_ok());
    let args = vec!["database", "close"];
    assert!(command.try_get_matches_from_mut(args).is_ok());
    let args = vec!["database", "close", "-s"];
    assert!(command.try_get_matches_from_mut(args).is_ok());
    let args = vec!["database", "close", "-s", "\"\""];
    assert!(command.try_get_matches_from_mut(args).is_err());

    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

    loop {
        terminal.draw(|f| {
            f.render_widget(Paragraph::new("q"), f.size());
        })?;

        if crossterm::event::poll(std::time::Duration::from_millis(250))? {
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                if key.code == crossterm::event::KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}