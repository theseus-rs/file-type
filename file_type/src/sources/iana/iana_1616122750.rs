use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1616122750: FileFormat = FileFormat {
    id: 1_616_122_750,
    source_type: SourceType::Iana,
    name: "pkcs10",
    extensions: &[],
    media_types: &["application/pkcs10"],
    internal_signatures: &[],
    related_formats: &[],
};
