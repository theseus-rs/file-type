use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3896998504: FileFormat = FileFormat {
    id: 3_896_998_504,
    source_type: SourceType::Iana,
    name: "encaprtp",
    extensions: &[],
    media_types: &["audio/encaprtp"],
    signatures: &[],
    related_formats: &[],
};
