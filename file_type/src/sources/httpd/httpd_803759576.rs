use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_803759576: FileFormat = FileFormat {
    id: 803_759_576,
    source_type: SourceType::Httpd,
    name: "mozilla xul xml",
    extensions: &["xul"],
    media_types: &["application/vnd.mozilla.xul+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
