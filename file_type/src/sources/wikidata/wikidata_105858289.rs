use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858289: FileFormat = FileFormat {
    id: 105_858_289,
    source_type: SourceType::Wikidata,
    name: "E:D Shipyard ship loadout",
    extensions: &["txt"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};
