use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_838250481: FileFormat = FileFormat {
    id: 838_250_481,
    source_type: SourceType::Iana,
    name: "G726-32",
    extensions: &[],
    media_types: &["audio/G726-32"],
    signatures: &[],
    related_formats: &[],
};
