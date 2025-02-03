use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1568676772: FileFormat = FileFormat {
    id: 1_568_676_772,
    source_type: SourceType::Httpd,
    name: "aiff",
    extensions: &["aif", "aiff", "aifc"],
    media_types: &["audio/x-aiff"],
    internal_signatures: &[],
    related_formats: &[],
};
