use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_887473226: FileFormat = FileFormat {
    id: 887_473_226,
    source_type: SourceType::Iana,
    name: "vorbis",
    extensions: &[],
    media_types: &["audio/vorbis"],
    signatures: &[],
    related_formats: &[],
};
