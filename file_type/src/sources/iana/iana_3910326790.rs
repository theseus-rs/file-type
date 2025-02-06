use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3910326790: FileFormat = FileFormat {
    id: 3_910_326_790,
    source_type: SourceType::Iana,
    name: "VMR-WB",
    extensions: &[],
    media_types: &["audio/VMR-WB"],
    signatures: &[],
    related_formats: &[],
};
