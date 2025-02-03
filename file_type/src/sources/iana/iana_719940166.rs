use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_719940166: FileFormat = FileFormat {
    id: 719_940_166,
    source_type: SourceType::Iana,
    name: "vnd.is-xpr",
    extensions: &[],
    media_types: &["application/vnd.is-xpr"],
    internal_signatures: &[],
    related_formats: &[],
};
