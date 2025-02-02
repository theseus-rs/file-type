use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_13037335205551783511: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "curl pcurl",
    extensions: &["pcurl"],
    media_types: &["application/vnd.curl.pcurl"],
    internal_signatures: &[],
    related_formats: &[],
};
