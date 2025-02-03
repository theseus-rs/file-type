use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4293424542: FileFormat = FileFormat {
    id: 4_293_424_542,
    source_type: SourceType::Iana,
    name: "G726-16",
    extensions: &[],
    media_types: &["audio/G726-16"],
    internal_signatures: &[],
    related_formats: &[],
};
