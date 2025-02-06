use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2899183557: FileFormat = FileFormat {
    id: 2_899_183_557,
    source_type: SourceType::Iana,
    name: "vnd.crypto-shade-file",
    extensions: &[],
    media_types: &["application/vnd.crypto-shade-file"],
    signatures: &[],
    related_formats: &[],
};
