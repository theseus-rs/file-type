use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_562877733: FileFormat = FileFormat {
    id: 562_877_733,
    source_type: SourceType::Httpd,
    name: "rn realmedia vbr",
    extensions: &["rmvb"],
    media_types: &["application/vnd.rn-realmedia-vbr"],
    internal_signatures: &[],
    related_formats: &[],
};
