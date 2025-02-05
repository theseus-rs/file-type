use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28445589: FileFormat = FileFormat {
    id: 28_445_589,
    source_type: SourceType::Wikidata,
    name: "AMOS AmBs",
    extensions: &["abk", "abs"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
