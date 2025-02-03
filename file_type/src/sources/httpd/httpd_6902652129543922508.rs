use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_6902652129543922508: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "sailingtracker track",
    extensions: &["st"],
    media_types: &["application/vnd.sailingtracker.track"],
    internal_signatures: &[],
    related_formats: &[],
};
