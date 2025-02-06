use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_118288651: FileFormat = FileFormat {
    id: 118_288_651,
    source_type: SourceType::Wikidata,
    name: "OnMark 2000 Project File",
    extensions: &["era"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
