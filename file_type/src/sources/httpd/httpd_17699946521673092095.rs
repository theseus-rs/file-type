use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_17699946521673092095: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "font ghostscript",
    extensions: &["gsf"],
    media_types: &["application/x-font-ghostscript"],
    internal_signatures: &[],
    related_formats: &[],
};
