use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_118288651: FileFormat = FileFormat {
    id: 118_288_651,
    source_type: SourceType::Wikidata,
    name: "OnMark 2000 Project File",
    extensions: &["era"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
