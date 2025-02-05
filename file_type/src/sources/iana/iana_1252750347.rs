use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1252750347: FileFormat = FileFormat {
    id: 1_252_750_347,
    source_type: SourceType::Iana,
    name: "fwdred",
    extensions: &[],
    media_types: &["text/fwdred"],
    signatures: &[],
    related_formats: &[],
};
