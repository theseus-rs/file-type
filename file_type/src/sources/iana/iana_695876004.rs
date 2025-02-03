use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_695876004: FileFormat = FileFormat {
    id: 695_876_004,
    source_type: SourceType::Iana,
    name: "vnd.sss-ntf",
    extensions: &[],
    media_types: &["application/vnd.sss-ntf"],
    internal_signatures: &[],
    related_formats: &[],
};
