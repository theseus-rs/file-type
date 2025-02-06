use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2434472940: FileFormat = FileFormat {
    id: 2_434_472_940,
    source_type: SourceType::Iana,
    name: "vnd.airzip.filesecure.azf",
    extensions: &[],
    media_types: &["application/vnd.airzip.filesecure.azf"],
    signatures: &[],
    related_formats: &[],
};
