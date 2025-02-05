use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2472612683: FileFormat = FileFormat {
    id: 2_472_612_683,
    source_type: SourceType::Httpd,
    name: "accpac simply aso",
    extensions: &["aso"],
    media_types: &["application/vnd.accpac.simply.aso"],
    signatures: &[],
    related_formats: &[],
};
