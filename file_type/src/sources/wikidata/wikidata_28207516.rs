use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28207516: FileFormat = FileFormat {
    id: 28_207_516,
    source_type: SourceType::Wikidata,
    name: "Word for DOS screen capture",
    extensions: &["scr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
