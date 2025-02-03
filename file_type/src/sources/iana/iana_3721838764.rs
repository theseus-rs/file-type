use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3721838764: FileFormat = FileFormat {
    id: 3_721_838_764,
    source_type: SourceType::Iana,
    name: "vnd.shana.informed.package",
    extensions: &[],
    media_types: &["application/vnd.shana.informed.package"],
    internal_signatures: &[],
    related_formats: &[],
};
