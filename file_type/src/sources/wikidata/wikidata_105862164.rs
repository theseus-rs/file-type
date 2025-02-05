use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862164: FileFormat = FileFormat {
    id: 105_862_164,
    source_type: SourceType::Wikidata,
    name: "Music Macro Language",
    extensions: &["mml"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
