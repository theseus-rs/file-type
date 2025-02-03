use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3541623162: FileFormat = FileFormat {
    id: 3_541_623_162,
    source_type: SourceType::Httpd,
    name: "mathematica",
    extensions: &["ma", "nb", "mb"],
    media_types: &["application/mathematica"],
    internal_signatures: &[],
    related_formats: &[],
};
