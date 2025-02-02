use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_6703499962162694527: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "claymore",
    extensions: &["cla"],
    media_types: &["application/vnd.claymore"],
    internal_signatures: &[],
    related_formats: &[],
};
