use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2769996487: FileFormat = FileFormat {
    id: 2_769_996_487,
    source_type: SourceType::Httpd,
    name: "apple mpegurl",
    extensions: &["m3u8"],
    media_types: &["application/vnd.apple.mpegurl"],
    internal_signatures: &[],
    related_formats: &[],
};
