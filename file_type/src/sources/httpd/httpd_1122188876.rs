use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1122188876: FileFormat = FileFormat {
    id: 1_122_188_876,
    source_type: SourceType::Httpd,
    name: "dra",
    extensions: &["dra"],
    media_types: &["audio/vnd.dra"],
    signatures: &[],
    related_formats: &[],
};
