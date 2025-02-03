use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3938229820: FileFormat = FileFormat {
    id: 3_938_229_820,
    source_type: SourceType::Httpd,
    name: "sus calendar",
    extensions: &["sus", "susp"],
    media_types: &["application/vnd.sus-calendar"],
    internal_signatures: &[],
    related_formats: &[],
};
