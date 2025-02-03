use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_564146210: FileFormat = FileFormat {
    id: 564_146_210,
    source_type: SourceType::Iana,
    name: "macwriteii",
    extensions: &[],
    media_types: &["application/macwriteii"],
    internal_signatures: &[],
    related_formats: &[],
};
