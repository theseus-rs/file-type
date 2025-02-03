use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_869924777: FileFormat = FileFormat {
    id: 869_924_777,
    source_type: SourceType::Httpd,
    name: "xwindowdump",
    extensions: &["xwd"],
    media_types: &["image/x-xwindowdump"],
    internal_signatures: &[],
    related_formats: &[],
};
