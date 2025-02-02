use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3732919740460660496: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "oma dd2 xml",
    extensions: &["dd2"],
    media_types: &["application/vnd.oma.dd2+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
