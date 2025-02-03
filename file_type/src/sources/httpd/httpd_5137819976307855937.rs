use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_5137819976307855937: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "smil xml",
    extensions: &["smi", "smil"],
    media_types: &["application/smil+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
