use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_85415606: FileFormat = FileFormat {
    id: 85_415_606,
    source_type: SourceType::Wikidata,
    name: "Sonic Scenarist Closed Caption Format",
    extensions: &["scc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
