use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_272413369: FileFormat = FileFormat {
    id: 272_413_369,
    source_type: SourceType::Iana,
    name: "RED",
    extensions: &[],
    media_types: &["audio/RED"],
    signatures: &[],
    related_formats: &[],
};
