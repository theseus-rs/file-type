use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_483127336: FileFormat = FileFormat {
    id: 483_127_336,
    source_type: SourceType::Httpd,
    name: "mpeg",
    extensions: &["mpga", "mp2", "mp2a", "mp3", "m2a", "m3a"],
    media_types: &["audio/mpeg"],
    internal_signatures: &[],
    related_formats: &[],
};
