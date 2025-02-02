use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_13380737628036455431: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "mp4",
    extensions: &["m4a", "mp4a"],
    media_types: &["audio/mp4"],
    internal_signatures: &[],
    related_formats: &[],
};
