use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_3551300: FileFormat = FileFormat {
    id: 3_551_300,
    source_type: SourceType::Wikidata,
    name: "Universal Subtitle Format",
    extensions: &["xml"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
