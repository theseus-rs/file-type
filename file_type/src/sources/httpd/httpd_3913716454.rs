use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3913716454: FileFormat = FileFormat {
    id: 3_913_716_454,
    source_type: SourceType::Httpd,
    name: "subrip",
    extensions: &["srt"],
    media_types: &["application/x-subrip"],
    signatures: &[],
    related_formats: &[],
};
