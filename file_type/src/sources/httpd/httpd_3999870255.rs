use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3999870255: FileFormat = FileFormat {
    id: 3_999_870_255,
    source_type: SourceType::Httpd,
    name: "mophun application",
    extensions: &["mpn"],
    media_types: &["application/vnd.mophun.application"],
    internal_signatures: &[],
    related_formats: &[],
};
