use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3963433202: FileFormat = FileFormat {
    id: 3_963_433_202,
    source_type: SourceType::Iana,
    name: "vnd.apexlang",
    extensions: &[],
    media_types: &["application/vnd.apexlang"],
    internal_signatures: &[],
    related_formats: &[],
};
