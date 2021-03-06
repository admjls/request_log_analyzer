use std::io;
pub mod log_events;
use self::log_events::*;

pub fn parse_line(line: Result<String, io::Error>) -> Result<LogEvent, &'static str> {
    match line {
        Err(_) => Err("Failed to read line."),
        Ok(ref line) if line.contains("->") => {
            Ok(LogEvent::Request(Request::new_from_log_line(line)?))
        }
        Ok(ref line) if line.contains("<-") => {
            Ok(LogEvent::Response(Response::new_from_log_line(line)?))
        }
        Ok(_) => Err("Line is neither a Request nor a Response"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line_request() {
        let line = "08/Apr/2016:09:58:47 +0200 [02] -> GET /content/some/other.html HTTP/1.1"
            .to_string();

        let event = match parse_line(Ok(line)).unwrap() {
            LogEvent::Request(request) => request,
            LogEvent::Response(_) => unreachable!(),
        };

        assert_eq!(event.id, 2);
    }

    #[test]
    fn test_parse_line_response() {
        let line = "08/Apr/2016:09:58:48 +0200 [05] <- 200 text/html 10ms".to_string();

        let event = match parse_line(Ok(line)).unwrap() {
            LogEvent::Request(_) => unreachable!(),
            LogEvent::Response(response) => response,
        };

        assert_eq!(event.id, 5);
    }

    #[test]
    fn test_parse_line_unrecognized() {
        let line = "08/Apr/2016:09:58:48 +0200 [05] XY 200 text/html 10ms".to_string();

        let event = parse_line(Ok(line));

        assert_eq!(event, Err("Line is neither a Request nor a Response"));
    }

    #[test]
    fn test_parse_line_with_response_arrow_in_url() {
        let line = "08/Apr/2016:09:58:47 +0200 [02] -> GET /content/<-.html HTTP/1.1".to_string();

        let _ = match parse_line(Ok(line)).unwrap() {
            LogEvent::Request(request) => request,
            LogEvent::Response(_) => unreachable!(),
        };
    }
}
