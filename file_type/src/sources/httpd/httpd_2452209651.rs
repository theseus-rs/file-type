use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2452209651: FileFormat = FileFormat {
    id: 2_452_209_651,
    source_type: SourceType::Httpd,
    name: "geospace",
    extensions: &["g3w"],
    media_types: &["application/vnd.geospace"],
    internal_signatures: &[],
    related_formats: &[],
};
