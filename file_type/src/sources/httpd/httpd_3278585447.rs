use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3278585447: FileFormat = FileFormat {
    id: 3_278_585_447,
    source_type: SourceType::Httpd,
    name: "groove vcard",
    extensions: &["vcg"],
    media_types: &["application/vnd.groove-vcard"],
    signatures: &[],
    related_formats: &[],
};
