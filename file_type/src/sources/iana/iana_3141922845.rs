use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3141922845: FileFormat = FileFormat {
    id: 3_141_922_845,
    source_type: SourceType::Iana,
    name: "MP1S",
    extensions: &[],
    media_types: &["video/MP1S"],
    signatures: &[],
    related_formats: &[],
};
