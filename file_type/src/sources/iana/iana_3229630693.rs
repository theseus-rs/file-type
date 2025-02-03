use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3229630693: FileFormat = FileFormat {
    id: 3_229_630_693,
    source_type: SourceType::Iana,
    name: "vnd.lotus-organizer",
    extensions: &[],
    media_types: &["application/vnd.lotus-organizer"],
    internal_signatures: &[],
    related_formats: &[],
};
