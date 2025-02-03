use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852952: FileFormat = FileFormat {
    id: 105_852_952,
    source_type: SourceType::Wikidata,
    name: "A'dam Music Composer Script (with rem)",
    extensions: &["scr"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
