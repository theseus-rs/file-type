use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2785079285: FileFormat = FileFormat {
    id: 2_785_079_285,
    source_type: SourceType::Httpd,
    name: "immervision ivp",
    extensions: &["ivp"],
    media_types: &["application/vnd.immervision-ivp"],
    internal_signatures: &[],
    related_formats: &[],
};
