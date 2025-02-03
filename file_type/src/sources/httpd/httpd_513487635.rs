use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_513487635: FileFormat = FileFormat {
    id: 513_487_635,
    source_type: SourceType::Httpd,
    name: "n3",
    extensions: &["n3"],
    media_types: &["text/n3"],
    internal_signatures: &[],
    related_formats: &[],
};
