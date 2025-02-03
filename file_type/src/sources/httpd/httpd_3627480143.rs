use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3627480143: FileFormat = FileFormat {
    id: 3_627_480_143,
    source_type: SourceType::Httpd,
    name: "webm",
    extensions: &["weba"],
    media_types: &["audio/webm"],
    internal_signatures: &[],
    related_formats: &[],
};
