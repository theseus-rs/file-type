use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1971868232: FileFormat = FileFormat {
    id: 1_971_868_232,
    source_type: SourceType::Iana,
    name: "mhas",
    extensions: &[],
    media_types: &["audio/mhas"],
    internal_signatures: &[],
    related_formats: &[],
};
