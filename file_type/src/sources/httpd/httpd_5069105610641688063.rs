use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_5069105610641688063: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "matroska",
    extensions: &["mka"],
    media_types: &["audio/x-matroska"],
    internal_signatures: &[],
    related_formats: &[],
};
