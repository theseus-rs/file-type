use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_109997309: FileFormat = FileFormat {
    id: 109_997_309,
    source_type: SourceType::Wikidata,
    name: "Stuffit Archive File, version 1.5",
    extensions: &["sit"],
    media_types: &["application/x-stuffit"],
    internal_signatures: &[],
    related_formats: &[],
};
