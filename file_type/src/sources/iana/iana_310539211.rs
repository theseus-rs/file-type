use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_310539211: FileFormat = FileFormat {
    id: 310_539_211,
    source_type: SourceType::Iana,
    name: "ogg",
    extensions: &[],
    media_types: &["audio/ogg"],
    internal_signatures: &[],
    related_formats: &[],
};
