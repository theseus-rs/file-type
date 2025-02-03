use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2346625709: FileFormat = FileFormat {
    id: 2_346_625_709,
    source_type: SourceType::Iana,
    name: "vnd.geonext",
    extensions: &[],
    media_types: &["application/vnd.geonext"],
    internal_signatures: &[],
    related_formats: &[],
};
