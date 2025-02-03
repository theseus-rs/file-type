use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4108897201: FileFormat = FileFormat {
    id: 4_108_897_201,
    source_type: SourceType::Httpd,
    name: "chess pgn",
    extensions: &["pgn"],
    media_types: &["application/x-chess-pgn"],
    internal_signatures: &[],
    related_formats: &[],
};
