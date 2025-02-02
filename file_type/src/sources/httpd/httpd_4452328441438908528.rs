use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4452328441438908528: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "ms project",
    extensions: &["mpp", "mpt"],
    media_types: &["application/vnd.ms-project"],
    internal_signatures: &[],
    related_formats: &[],
};
