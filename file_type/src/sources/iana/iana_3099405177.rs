use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3099405177: FileFormat = FileFormat {
    id: 3_099_405_177,
    source_type: SourceType::Iana,
    name: "vnd.everad.plj",
    extensions: &[],
    media_types: &["audio/vnd.everad.plj"],
    signatures: &[],
    related_formats: &[],
};
