use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3587406333: FileFormat = FileFormat {
    id: 3_587_406_333,
    source_type: SourceType::Iana,
    name: "MELP",
    extensions: &[],
    media_types: &["audio/MELP"],
    signatures: &[],
    related_formats: &[],
};
