use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_51801746: FileFormat = FileFormat {
    id: 51_801_746,
    source_type: SourceType::Wikidata,
    name: "Stationery for Mac OS X",
    extensions: &["doc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
