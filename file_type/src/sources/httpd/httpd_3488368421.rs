use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3488368421: FileFormat = FileFormat {
    id: 3_488_368_421,
    source_type: SourceType::Httpd,
    name: "msmoney",
    extensions: &["mny"],
    media_types: &["application/x-msmoney"],
    internal_signatures: &[],
    related_formats: &[],
};
