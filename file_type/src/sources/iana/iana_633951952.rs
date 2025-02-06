use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_633951952: FileFormat = FileFormat {
    id: 633_951_952,
    source_type: SourceType::Iana,
    name: "vnd.wmc",
    extensions: &[],
    media_types: &["application/vnd.wmc"],
    signatures: &[],
    related_formats: &[],
};
