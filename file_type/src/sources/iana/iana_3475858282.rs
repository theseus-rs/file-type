use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3475858282: FileFormat = FileFormat {
    id: 3_475_858_282,
    source_type: SourceType::Iana,
    name: "vnd.wap.sic",
    extensions: &[],
    media_types: &["application/vnd.wap.sic"],
    internal_signatures: &[],
    related_formats: &[],
};
