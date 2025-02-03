use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2551238913: FileFormat = FileFormat {
    id: 2_551_238_913,
    source_type: SourceType::Httpd,
    name: "prs btif",
    extensions: &["btif"],
    media_types: &["image/prs.btif"],
    internal_signatures: &[],
    related_formats: &[],
};
