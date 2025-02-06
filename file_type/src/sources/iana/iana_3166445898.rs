use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3166445898: FileFormat = FileFormat {
    id: 3_166_445_898,
    source_type: SourceType::Iana,
    name: "mesh",
    extensions: &[],
    media_types: &["model/mesh"],
    signatures: &[],
    related_formats: &[],
};
