use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_18176455083221241937: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "gtw",
    extensions: &["gtw"],
    media_types: &["model/vnd.gtw"],
    internal_signatures: &[],
    related_formats: &[],
};
