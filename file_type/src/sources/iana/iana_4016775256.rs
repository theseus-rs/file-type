use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4016775256: FileFormat = FileFormat {
    id: 4_016_775_256,
    source_type: SourceType::Iana,
    name: "linkset",
    extensions: &[],
    media_types: &["application/linkset"],
    internal_signatures: &[],
    related_formats: &[],
};
