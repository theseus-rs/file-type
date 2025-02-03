use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2988350570: FileFormat = FileFormat {
    id: 2_988_350_570,
    source_type: SourceType::Httpd,
    name: "font ghostscript",
    extensions: &["gsf"],
    media_types: &["application/x-font-ghostscript"],
    internal_signatures: &[],
    related_formats: &[],
};
