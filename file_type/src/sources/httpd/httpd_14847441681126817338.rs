use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_14847441681126817338: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "chess pgn",
    extensions: &["pgn"],
    media_types: &["application/x-chess-pgn"],
    internal_signatures: &[],
    related_formats: &[],
};
