use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2388933680: FileFormat = FileFormat {
    id: 2_388_933_680,
    source_type: SourceType::Httpd,
    name: "t3vm image",
    extensions: &["t3"],
    media_types: &["application/x-t3vm-image"],
    internal_signatures: &[],
    related_formats: &[],
};
