use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_66711987: FileFormat = FileFormat {
    id: 66_711_987,
    source_type: SourceType::Wikidata,
    name: "Word Macro-Enabled Template",
    extensions: &["dotm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
