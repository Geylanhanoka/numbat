use numbat::markup::{FormatType, FormattedString, Formatter};

use colored::Colorize;

pub struct ANSIFormatter;

impl Formatter for ANSIFormatter {
    fn format_part(
        &self,
        FormattedString(_output_type, format_type, text): &FormattedString,
    ) -> String {
        (match format_type {
            FormatType::Text => text.normal(),
            FormatType::Keyword => text.magenta(),
            FormatType::Value => text.yellow(),
            FormatType::Unit => text.cyan(),
            FormatType::Identifier => text.red(),
            FormatType::TypeIdentifier => text.bright_yellow(),
            FormatType::Operator => text.bold(),
            FormatType::Decorator => text.yellow(),
        })
        .to_string()
    }
}
