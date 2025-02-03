use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4246575942: FileFormat = FileFormat {
    id: 4_246_575_942,
    source_type: SourceType::Httpd,
    name: "ms playready media pyv",
    extensions: &["pyv"],
    media_types: &["video/vnd.ms-playready.media.pyv"],
    internal_signatures: &[],
    related_formats: &[],
};
