use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4129216099: FileFormat = FileFormat {
    id: 4_129_216_099,
    source_type: SourceType::Httpd,
    name: "jpeg",
    extensions: &["jpgv"],
    media_types: &["video/jpeg"],
    internal_signatures: &[],
    related_formats: &[],
};
