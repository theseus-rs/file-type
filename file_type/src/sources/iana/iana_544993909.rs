use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_544993909: FileFormat = FileFormat {
    id: 544_993_909,
    source_type: SourceType::Iana,
    name: "cdni",
    extensions: &[],
    media_types: &["application/cdni"],
    signatures: &[],
    related_formats: &[],
};
