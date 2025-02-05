use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_964127301: FileFormat = FileFormat {
    id: 964_127_301,
    source_type: SourceType::Iana,
    name: "pskc+xml",
    extensions: &[],
    media_types: &["application/pskc+xml"],
    signatures: &[],
    related_formats: &[],
};
