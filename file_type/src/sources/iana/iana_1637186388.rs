use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1637186388: FileFormat = FileFormat {
    id: 1_637_186_388,
    source_type: SourceType::Iana,
    name: "vnd.music-niff",
    extensions: &[],
    media_types: &["application/vnd.music-niff"],
    signatures: &[],
    related_formats: &[],
};
