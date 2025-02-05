use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1968309032: FileFormat = FileFormat {
    id: 1_968_309_032,
    source_type: SourceType::Iana,
    name: "vnd.recordare.musicxml",
    extensions: &[],
    media_types: &["application/vnd.recordare.musicxml"],
    signatures: &[],
    related_formats: &[],
};
