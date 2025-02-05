use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2434472940: FileFormat = FileFormat {
    id: 2_434_472_940,
    source_type: SourceType::Httpd,
    name: "airzip filesecure azf",
    extensions: &["azf"],
    media_types: &["application/vnd.airzip.filesecure.azf"],
    signatures: &[],
    related_formats: &[],
};
