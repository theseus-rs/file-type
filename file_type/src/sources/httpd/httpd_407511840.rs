use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_407511840: FileFormat = FileFormat {
    id: 407_511_840,
    source_type: SourceType::Httpd,
    name: "mscardfile",
    extensions: &["crd"],
    media_types: &["application/x-mscardfile"],
    internal_signatures: &[],
    related_formats: &[],
};
