use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1650765870: FileFormat = FileFormat {
    id: 1_650_765_870,
    source_type: SourceType::Httpd,
    name: "mfmp",
    extensions: &["mfm"],
    media_types: &["application/vnd.mfmp"],
    signatures: &[],
    related_formats: &[],
};
