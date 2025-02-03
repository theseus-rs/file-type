use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4090732028: FileFormat = FileFormat {
    id: 4_090_732_028,
    source_type: SourceType::Iana,
    name: "EVS",
    extensions: &[],
    media_types: &["audio/EVS"],
    internal_signatures: &[],
    related_formats: &[],
};
