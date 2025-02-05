use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1541299051: FileFormat = FileFormat {
    id: 1_541_299_051,
    source_type: SourceType::Iana,
    name: "vnd.oasis.opendocument.image-template",
    extensions: &[],
    media_types: &["application/vnd.oasis.opendocument.image-template"],
    signatures: &[],
    related_formats: &[],
};
