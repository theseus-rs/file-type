use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3065629133: FileFormat = FileFormat {
    id: 3_065_629_133,
    source_type: SourceType::Iana,
    name: "vnd.cloanto.rp9",
    extensions: &[],
    media_types: &["application/vnd.cloanto.rp9"],
    internal_signatures: &[],
    related_formats: &[],
};
