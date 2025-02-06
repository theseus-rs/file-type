use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_912766839: FileFormat = FileFormat {
    id: 912_766_839,
    source_type: SourceType::Iana,
    name: "G7291",
    extensions: &[],
    media_types: &["audio/G7291"],
    signatures: &[],
    related_formats: &[],
};
