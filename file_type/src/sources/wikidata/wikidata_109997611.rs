use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_109997611: FileFormat = FileFormat {
    id: 109_997_611,
    source_type: SourceType::Wikidata,
    name: "Stuffit Archive File, version 1.6-4.5",
    extensions: &["sit"],
    media_types: &["application/x-stuffit"],
    signatures: &[],
    related_formats: &[],
};
