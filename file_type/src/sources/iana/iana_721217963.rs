use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_721217963: FileFormat = FileFormat {
    id: 721_217_963,
    source_type: SourceType::Iana,
    name: "vnd.neurolanguage.nlu",
    extensions: &[],
    media_types: &["application/vnd.neurolanguage.nlu"],
    internal_signatures: &[],
    related_formats: &[],
};
