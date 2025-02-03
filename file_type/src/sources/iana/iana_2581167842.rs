use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2581167842: FileFormat = FileFormat {
    id: 2_581_167_842,
    source_type: SourceType::Iana,
    name: "EVRCNW",
    extensions: &[],
    media_types: &["audio/EVRCNW"],
    internal_signatures: &[],
    related_formats: &[],
};
