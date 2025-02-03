use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_123378450: FileFormat = FileFormat {
    id: 123_378_450,
    source_type: SourceType::Wikidata,
    name: "TrueSpace Selection file",
    extensions: &["sel"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
