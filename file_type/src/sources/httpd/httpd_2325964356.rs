use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2325964356: FileFormat = FileFormat {
    id: 2_325_964_356,
    source_type: SourceType::Httpd,
    name: "c",
    extensions: &["c", "cc", "cxx", "cpp", "h", "hh", "dic"],
    media_types: &["text/x-c"],
    internal_signatures: &[],
    related_formats: &[],
};
