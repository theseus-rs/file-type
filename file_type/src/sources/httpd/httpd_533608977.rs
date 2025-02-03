use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_533608977: FileFormat = FileFormat {
    id: 533_608_977,
    source_type: SourceType::Httpd,
    name: "patch ops error xml",
    extensions: &["xer"],
    media_types: &["application/patch-ops-error+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
