use std::path::PathBuf;
use tooka_core::rules_file::RulesFile;

pub struct Tooka {
    pub state: State,
    pub sort_path: Option<PathBuf>,
    pub rules_file: RulesFile,
}

#[derive(Debug)]
pub enum State {
    Home { dry_run: bool },
    Settings,
    Rules,
    Sorting,
    Results,
}

#[derive(Debug, Clone)]
pub enum Message {
    OpenMain,
    OpenSettings,
    OpenRules,
    StartSorting,
    OpenResults,
    ToggleDryRun,
    AddRule,
    RemoveRule,
    ExportRule,
    DownloadPdf,
    Shutdown,
    OpenTerms,
    Open {
        path: PathBuf,
    },
    PopUp {
        severity: Severity,
        title: String,
        description: String,
    },
    Error {
        case: ErrorCase,
    },
    None,
}

#[derive(Debug, Clone)]
pub enum Severity {
    Confirm { yes: Box<Message>, no: Box<Message> },
    Warning { confirm: Box<Message> },
    Error { confirm: Box<Message> },
}

#[derive(Debug, Clone)]
pub enum ErrorCase {
    Critical { message: String },
    Warning { message: String },
}

impl Tooka {
    pub fn new() -> Self {
        tooka_core::context::init_config().expect("Failed to initialize config");
        tooka_core::context::init_rules_file().expect("Failed to initialize rules file");
        tooka_core::logger::init_logger().expect("Failed to initialize logger");
        let config = tooka_core::context::get_locked_config().expect("Failed to get locked config");
        let rules_file =
            tooka_core::context::get_locked_rules_file().expect("Failed to get locked rules file");
        let default_sort_path = config.source_folder.as_path();

        Self {
            state: State::Home { dry_run: false },
            sort_path: Some(default_sort_path.to_path_buf()),
            rules_file: rules_file.clone(),
        }
    }

    pub fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::OpenMain => {
                self.state = State::Home { dry_run: false };
                iced::Task::none()
            }
            Message::OpenSettings => {
                self.state = State::Settings;
                iced::Task::none()
            }
            Message::OpenRules => {
                self.state = State::Rules;
                iced::Task::none()
            }
            Message::StartSorting => {
                self.state = State::Sorting;
                iced::Task::none()
            }
            Message::ToggleDryRun => {
                if let State::Home { dry_run } = &mut self.state {
                    *dry_run = !*dry_run;
                }
                iced::Task::none()
            }
            Message::OpenResults => {
                self.state = State::Results;
                iced::Task::none()
            }
            Message::AddRule => iced::Task::none(),
            Message::RemoveRule => iced::Task::none(),
            Message::ExportRule => iced::Task::none(),
            Message::DownloadPdf => iced::Task::none(),
            Message::OpenTerms => iced::Task::none(),
            Message::Shutdown => std::process::exit(0),
            Message::Open { path } => iced::Task::perform(
                async {
                    log::info!("Opening {}...", path.to_string_lossy());
                    match open::that(path) {
                        Ok(_) => Message::None,
                        Err(message) => Message::Error {
                            case: ErrorCase::Warning {
                                message: message.to_string(),
                            },
                        },
                    }
                },
                |result| result,
            ),
            Message::PopUp {
                severity,
                title,
                description,
            } => iced::Task::done({
                let dialog = rfd::MessageDialog::new()
                    .set_title(title)
                    .set_description(description);

                match severity {
                    Severity::Confirm { yes, no } => {
                        match dialog
                            .set_buttons(rfd::MessageButtons::YesNo)
                            .set_level(rfd::MessageLevel::Info)
                            .show()
                        {
                            rfd::MessageDialogResult::Yes => *yes,
                            rfd::MessageDialogResult::No => *no,
                            _ => Message::None,
                        }
                    }
                    Severity::Warning { confirm } => {
                        match dialog
                            .set_buttons(rfd::MessageButtons::Ok)
                            .set_level(rfd::MessageLevel::Warning)
                            .show()
                        {
                            rfd::MessageDialogResult::Ok => *confirm,
                            _ => Message::None,
                        }
                    }
                    Severity::Error { confirm } => {
                        match dialog
                            .set_buttons(rfd::MessageButtons::Ok)
                            .set_level(rfd::MessageLevel::Error)
                            .show()
                        {
                            rfd::MessageDialogResult::Ok => *confirm,
                            _ => Message::None,
                        }
                    }
                }
            }),
            Message::Error { case } => match case {
                ErrorCase::Critical { message } => iced::Task::done({
                    Message::PopUp {
                        severity: Severity::Error {
                            confirm: Box::new(Message::Shutdown),
                        },
                        title: "Critical Error".to_string(),
                        description: message,
                    }
                }),
                ErrorCase::Warning { message } => iced::Task::done({
                    log::warn!("{message}");
                    Message::PopUp {
                        severity: Severity::Warning {
                            confirm: Box::new(Message::None),
                        },
                        title: "Warning".to_string(),
                        description: message,
                    }
                }),
            },
            _ => iced::Task::none(),
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        match &self.state {
            State::Home { dry_run } => self.home(),
            State::Settings => self.settings(),
            State::Rules => self.rules(),
            State::Sorting => self.sorting(),
            State::Results => self.results(),
        }
    }
}

impl Default for Tooka {
    fn default() -> Self {
        Self::new()
    }
}

pub fn wrap(padding: u16, element: iced::Element<Message>) -> iced::Element<Message> {
    iced::widget::Container::new(element)
        .padding(padding)
        .into()
}
