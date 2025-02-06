use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_195837457: FileFormat = FileFormat {
    id: 195_837_457,
    source_type: SourceType::Iana,
    name: "dls",
    extensions: &[],
    media_types: &["audio/dls"],
    signatures: &[],
    related_formats: &[],
};
