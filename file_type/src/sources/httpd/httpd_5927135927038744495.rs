use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_5927135927038744495: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "lost xml",
    extensions: &["lostxml"],
    media_types: &["application/lost+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
