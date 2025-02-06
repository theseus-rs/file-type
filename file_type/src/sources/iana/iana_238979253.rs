use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_238979253: FileFormat = FileFormat {
    id: 238_979_253,
    source_type: SourceType::Iana,
    name: "vnd.sqlite3",
    extensions: &[],
    media_types: &["application/vnd.sqlite3"],
    signatures: &[],
    related_formats: &[],
};
