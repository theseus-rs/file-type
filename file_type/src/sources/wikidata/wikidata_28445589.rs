use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28445589: FileFormat = FileFormat {
    id: 28_445_589,
    source_type: SourceType::Wikidata,
    name: "AMOS AmBs",
    extensions: &["abk", "abs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
