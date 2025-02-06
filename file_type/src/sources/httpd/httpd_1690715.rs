use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1690715: FileFormat = FileFormat {
    id: 1_690_715,
    source_type: SourceType::Httpd,
    name: "iccprofile",
    extensions: &["icc", "icm"],
    media_types: &["application/vnd.iccprofile"],
    signatures: &[],
    related_formats: &[],
};
