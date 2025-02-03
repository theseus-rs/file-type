use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_437539308: FileFormat = FileFormat {
    id: 437_539_308,
    source_type: SourceType::Httpd,
    name: "cfs compressed",
    extensions: &["cfs"],
    media_types: &["application/x-cfs-compressed"],
    internal_signatures: &[],
    related_formats: &[],
};
