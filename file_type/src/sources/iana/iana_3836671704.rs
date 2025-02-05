use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3836671704: FileFormat = FileFormat {
    id: 3_836_671_704,
    source_type: SourceType::Iana,
    name: "index.response",
    extensions: &[],
    media_types: &["application/index.response"],
    signatures: &[],
    related_formats: &[],
};
