use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1220007935: FileFormat = FileFormat {
    id: 1_220_007_935,
    source_type: SourceType::Httpd,
    name: "richtext",
    extensions: &["rtx"],
    media_types: &["text/richtext"],
    internal_signatures: &[],
    related_formats: &[],
};
