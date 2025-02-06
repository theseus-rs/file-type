use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2395536891: FileFormat = FileFormat {
    id: 2_395_536_891,
    source_type: SourceType::Httpd,
    name: "ms ims",
    extensions: &["ims"],
    media_types: &["application/vnd.ms-ims"],
    signatures: &[],
    related_formats: &[],
};
