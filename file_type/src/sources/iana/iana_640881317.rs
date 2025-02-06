use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_640881317: FileFormat = FileFormat {
    id: 640_881_317,
    source_type: SourceType::Iana,
    name: "moss-signature",
    extensions: &[],
    media_types: &["application/moss-signature"],
    signatures: &[],
    related_formats: &[],
};
