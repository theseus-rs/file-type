use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29151590: FileFormat = FileFormat {
    id: 29_151_590,
    source_type: SourceType::Wikidata,
    name: "Redcode oBJect",
    extensions: &["rbj"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
