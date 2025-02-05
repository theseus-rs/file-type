use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2760154822: FileFormat = FileFormat {
    id: 2_760_154_822,
    source_type: SourceType::Httpd,
    name: "epson esf",
    extensions: &["esf"],
    media_types: &["application/vnd.epson.esf"],
    signatures: &[],
    related_formats: &[],
};
