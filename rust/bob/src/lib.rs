enum Response {
    Question,
    Silence,
    Yelling,
    YellingQuestion,
    Other,
}

impl Response {
    fn into(self) -> &'static str {
        return match self {
            Self::Question => "Sure.",
            Self::Silence => "Fine. Be that way!",
            Self::Yelling => "Whoa, chill out!",
            Self::YellingQuestion => "Calm down, I know what I'm doing!",
            Self::Other => "Whatever.",
        };
    }
}

fn is_question(message: &str) -> bool {
    return message.trim().ends_with('?');
}

fn is_silence(message: &str) -> bool {
    return message.trim().is_empty();
}

fn is_yelling(message: &str) -> bool {
    return message.chars().any(|c| c.is_alphabetic()) && message == message.to_uppercase();
}


fn is_yelling_question(message: &str) -> bool {
    return is_yelling(message) && is_question(message);
}

pub fn reply(message: &str) -> &str {
    return match message {
        m if is_silence(m) => Response::Silence,
        m if is_yelling_question(m) => Response::YellingQuestion,
        m if is_yelling(m) => Response::Yelling,
        m if is_question(m) => Response::Question,
        _ => Response::Other,
    }.into();
}
