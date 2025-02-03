use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_942626101: FileFormat = FileFormat {
    id: 942_626_101,
    source_type: SourceType::Iana,
    name: "vnd.ms-3mfdocument",
    extensions: &[],
    media_types: &["application/vnd.ms-3mfdocument"],
    internal_signatures: &[],
    related_formats: &[],
};
