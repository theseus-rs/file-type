use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_734953880: FileFormat = FileFormat {
    id: 734_953_880,
    source_type: SourceType::Iana,
    name: "vorbis-config",
    extensions: &[],
    media_types: &["audio/vorbis-config"],
    signatures: &[],
    related_formats: &[],
};
