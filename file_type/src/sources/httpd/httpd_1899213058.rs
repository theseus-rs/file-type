use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1899213058: FileFormat = FileFormat {
    id: 1_899_213_058,
    source_type: SourceType::Httpd,
    name: "mobius daf",
    extensions: &["daf"],
    media_types: &["application/vnd.mobius.daf"],
    internal_signatures: &[],
    related_formats: &[],
};
