use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_985613281: FileFormat = FileFormat {
    id: 985_613_281,
    source_type: SourceType::Iana,
    name: "vnd.SimTech-MindMapper",
    extensions: &[],
    media_types: &["application/vnd.SimTech-MindMapper"],
    signatures: &[],
    related_formats: &[],
};
