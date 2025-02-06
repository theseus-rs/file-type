use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_933758247: FileFormat = FileFormat {
    id: 933_758_247,
    source_type: SourceType::Iana,
    name: "vc+cose",
    extensions: &[],
    media_types: &["application/vc+cose"],
    signatures: &[],
    related_formats: &[],
};
