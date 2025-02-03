use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2734779849: FileFormat = FileFormat {
    id: 2_734_779_849,
    source_type: SourceType::Httpd,
    name: "gtw",
    extensions: &["gtw"],
    media_types: &["model/vnd.gtw"],
    internal_signatures: &[],
    related_formats: &[],
};
