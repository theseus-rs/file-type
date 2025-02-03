use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_12491493047811765531: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "dgc compressed",
    extensions: &["dgc"],
    media_types: &["application/x-dgc-compressed"],
    internal_signatures: &[],
    related_formats: &[],
};
