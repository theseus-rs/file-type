use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2397780198454870296: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "mathematica",
    extensions: &["ma", "nb", "mb"],
    media_types: &["application/mathematica"],
    internal_signatures: &[],
    related_formats: &[],
};
