use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3539896745: FileFormat = FileFormat {
    id: 3_539_896_745,
    source_type: SourceType::Httpd,
    name: "oma dd2 xml",
    extensions: &["dd2"],
    media_types: &["application/vnd.oma.dd2+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
