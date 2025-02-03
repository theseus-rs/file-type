use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105862164: FileFormat = FileFormat {
    id: 105_862_164,
    source_type: SourceType::Wikidata,
    name: "Music Macro Language",
    extensions: &["mml"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
