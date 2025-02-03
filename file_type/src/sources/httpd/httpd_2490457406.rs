use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2490457406: FileFormat = FileFormat {
    id: 2_490_457_406,
    source_type: SourceType::Httpd,
    name: "font linux psf",
    extensions: &["psf"],
    media_types: &["application/x-font-linux-psf"],
    internal_signatures: &[],
    related_formats: &[],
};
