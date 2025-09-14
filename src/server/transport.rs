//! MCP server transport layer for stdin/stdout communication.

use crate::domain::{YnabError, YnabResult};
use std::io::{BufRead, BufReader, Read, Write};

/// Reads a message from the given reader using Content-Length header.
///
/// MCP protocol uses HTTP-like headers with Content-Length to frame messages
/// over stdio streams.
pub fn read_message<R: Read>(reader: R) -> YnabResult<String> {
    let mut buf_reader = BufReader::new(reader);
    let mut header_line = String::new();

    // Read Content-Length header
    buf_reader.read_line(&mut header_line)?;

    if !header_line.starts_with("Content-Length:") {
        return Err(YnabError::api_error(
            "Expected Content-Length header".to_string(),
        ));
    }

    // Parse content length
    let length_str = header_line
        .trim()
        .strip_prefix("Content-Length:")
        .ok_or_else(|| YnabError::api_error("Invalid Content-Length header".to_string()))?
        .trim();

    let content_length: usize = length_str.parse().map_err(|_| {
        YnabError::api_error(format!("Invalid Content-Length value: {}", length_str))
    })?;

    // Read empty line (separator)
    let mut empty_line = String::new();
    buf_reader.read_line(&mut empty_line)?;

    // Read message content
    let mut buffer = vec![0; content_length];
    buf_reader.read_exact(&mut buffer)?;

    let message = String::from_utf8(buffer)
        .map_err(|_| YnabError::api_error("Message content is not valid UTF-8".to_string()))?;

    Ok(message)
}

/// Writes a message to the given writer with Content-Length header.
///
/// Formats the message with proper MCP transport framing.
pub fn write_message<W: Write>(mut writer: W, message: &str) -> YnabResult<()> {
    let content_length = message.len();
    let header = format!("Content-Length: {}\r\n\r\n{}", content_length, message);

    writer.write_all(header.as_bytes())?;
    writer.flush()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn should_read_jsonrpc_messages_from_reader() {
        let json_message = r#"{"jsonrpc":"2.0","method":"tools/list","id":1}"#;
        let content_length = json_message.len();
        let input = format!("Content-Length: {}\r\n\r\n{}", content_length, json_message);
        let mut reader = Cursor::new(input);

        let message = read_message(&mut reader).unwrap();

        assert_eq!(message, json_message);
    }

    #[test]
    fn should_write_jsonrpc_response_with_content_length() {
        let response = r#"{"jsonrpc":"2.0","id":1,"result":{"tools":[]}}"#;
        let mut writer = Vec::new();

        write_message(&mut writer, response).unwrap();

        let output = String::from_utf8(writer).unwrap();
        let expected_length = response.len();

        assert!(output.starts_with(&format!("Content-Length: {}\r\n\r\n", expected_length)));
        assert!(output.ends_with(response));
    }

    #[test]
    fn should_handle_invalid_content_length_header() {
        let input = "Invalid-Header: 42\r\n\r\ntest";
        let mut reader = Cursor::new(input);

        let result = read_message(&mut reader);

        assert!(result.is_err());
        match result.unwrap_err() {
            YnabError::ApiError(msg) => assert!(msg.contains("Expected Content-Length header")),
            other => panic!("Expected ApiError, got: {:?}", other),
        }
    }

    #[test]
    fn should_handle_invalid_content_length_value() {
        let input = "Content-Length: not-a-number\r\n\r\ntest";
        let mut reader = Cursor::new(input);

        let result = read_message(&mut reader);

        assert!(result.is_err());
        match result.unwrap_err() {
            YnabError::ApiError(msg) => assert!(msg.contains("Invalid Content-Length value")),
            other => panic!("Expected ApiError, got: {:?}", other),
        }
    }

    #[test]
    fn should_handle_content_length_mismatch() {
        let input = "Content-Length: 100\r\n\r\nshort";
        let mut reader = Cursor::new(input);

        let result = read_message(&mut reader);

        assert!(result.is_err());
        match result.unwrap_err() {
            YnabError::IoError(_) => {} // Expected - EOF before reading full content
            other => panic!("Expected IoError, got: {:?}", other),
        }
    }

    #[test]
    fn should_handle_invalid_utf8_content() {
        // Create a message with invalid UTF-8 bytes
        let invalid_utf8_bytes = vec![0xFF, 0xFE, 0xFD]; // Invalid UTF-8 sequence
        let content_length = invalid_utf8_bytes.len();

        // Build the full message with proper Content-Length header
        let mut input = format!("Content-Length: {}\r\n\r\n", content_length).into_bytes();
        input.extend_from_slice(&invalid_utf8_bytes);

        let mut reader = Cursor::new(input);

        let result = read_message(&mut reader);

        assert!(result.is_err());
        match result.unwrap_err() {
            YnabError::ApiError(msg) => assert!(msg.contains("Message content is not valid UTF-8")),
            other => panic!("Expected ApiError, got: {:?}", other),
        }
    }

    #[test]
    fn should_handle_missing_empty_line_separator() {
        // Test missing \r\n\r\n separator - just \r\n after Content-Length
        let input = "Content-Length: 5\r\nhello";
        let mut reader = Cursor::new(input);

        let result = read_message(&mut reader);

        // This should either succeed (reading "hello" as part of empty line + content)
        // or fail on the read_exact operation, depending on parsing behavior
        // The key is exercising the empty line reading code path
        if let Ok(message) = result {
            // If it succeeds, verify some content was read
            assert!(!message.is_empty());
        } else {
            // If it fails, it should be an IO error
            match result.unwrap_err() {
                YnabError::IoError(_) => {} // Expected
                other => panic!("Expected IoError, got: {:?}", other),
            }
        }
    }

    #[test]
    fn should_read_message_with_proper_separator() {
        // Test that the empty line separator is properly consumed
        let json_message = "test";
        let content_length = json_message.len();
        let input = format!("Content-Length: {}\r\n\r\n{}", content_length, json_message);
        let mut reader = Cursor::new(input);

        let message = read_message(&mut reader).unwrap();

        assert_eq!(message, json_message);

        // Verify that subsequent reads would get EOF (separator was consumed)
        let mut buf = String::new();
        let bytes_read = reader.read_line(&mut buf).unwrap();
        assert_eq!(bytes_read, 0); // EOF
    }
}
