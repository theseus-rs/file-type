use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3460806808: FileFormat = FileFormat {
    id: 3_460_806_808,
    source_type: SourceType::Httpd,
    name: "xpixmap",
    extensions: &["xpm"],
    media_types: &["image/x-xpixmap"],
    internal_signatures: &[],
    related_formats: &[],
};
