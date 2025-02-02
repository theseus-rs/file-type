use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4712921981521909307: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "xwindowdump",
    extensions: &["xwd"],
    media_types: &["image/x-xwindowdump"],
    internal_signatures: &[],
    related_formats: &[],
};
