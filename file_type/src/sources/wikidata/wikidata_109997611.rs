use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_109997611: FileFormat = FileFormat {
    id: 109_997_611,
    source_type: SourceType::Wikidata,
    name: "Stuffit Archive File, version 1.6-4.5",
    extensions: &["sit"],
    media_types: &["application/x-stuffit"],
    internal_signatures: &[],
    related_formats: &[],
};
