use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3335845233: FileFormat = FileFormat {
    id: 3_335_845_233,
    source_type: SourceType::Iana,
    name: "ODA",
    extensions: &[],
    media_types: &["application/ODA"],
    internal_signatures: &[],
    related_formats: &[],
};
