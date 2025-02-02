use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_17376429027797396733: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "silk",
    extensions: &["sil"],
    media_types: &["audio/silk"],
    internal_signatures: &[],
    related_formats: &[],
};
