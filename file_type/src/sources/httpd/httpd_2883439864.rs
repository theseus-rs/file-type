use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2883439864: FileFormat = FileFormat {
    id: 2_883_439_864,
    source_type: SourceType::Httpd,
    name: "enliven",
    extensions: &["nml"],
    media_types: &["application/vnd.enliven"],
    signatures: &[],
    related_formats: &[],
};
