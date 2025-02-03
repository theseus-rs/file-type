use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1626546553: FileFormat = FileFormat {
    id: 1_626_546_553,
    source_type: SourceType::Httpd,
    name: "isac fcs",
    extensions: &["fcs"],
    media_types: &["application/vnd.isac.fcs"],
    internal_signatures: &[],
    related_formats: &[],
};
