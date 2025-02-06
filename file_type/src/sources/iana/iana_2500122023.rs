use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2500122023: FileFormat = FileFormat {
    id: 2_500_122_023,
    source_type: SourceType::Iana,
    name: "otf",
    extensions: &[],
    media_types: &["font/otf"],
    signatures: &[],
    related_formats: &[],
};
