use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_17531001443158399273: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "fst",
    extensions: &["fst"],
    media_types: &["image/vnd.fst"],
    internal_signatures: &[],
    related_formats: &[],
};
