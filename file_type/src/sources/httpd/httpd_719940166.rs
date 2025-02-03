use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_719940166: FileFormat = FileFormat {
    id: 719_940_166,
    source_type: SourceType::Httpd,
    name: "is xpr",
    extensions: &["xpr"],
    media_types: &["application/vnd.is-xpr"],
    internal_signatures: &[],
    related_formats: &[],
};
