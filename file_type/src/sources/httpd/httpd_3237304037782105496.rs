use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3237304037782105496: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "ms playready media pyv",
    extensions: &["pyv"],
    media_types: &["video/vnd.ms-playready.media.pyv"],
    internal_signatures: &[],
    related_formats: &[],
};
