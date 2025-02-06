use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3556509107: FileFormat = FileFormat {
    id: 3_556_509_107,
    source_type: SourceType::Iana,
    name: "clearmode",
    extensions: &[],
    media_types: &["audio/clearmode"],
    signatures: &[],
    related_formats: &[],
};
