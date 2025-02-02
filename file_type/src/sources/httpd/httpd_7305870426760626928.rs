use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_7305870426760626928: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "metalink xml",
    extensions: &["metalink"],
    media_types: &["application/metalink+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
