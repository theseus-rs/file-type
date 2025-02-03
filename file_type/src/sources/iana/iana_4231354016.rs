use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4231354016: FileFormat = FileFormat {
    id: 4_231_354_016,
    source_type: SourceType::Iana,
    name: "32kadpcm",
    extensions: &[],
    media_types: &["audio/32kadpcm"],
    internal_signatures: &[],
    related_formats: &[],
};
