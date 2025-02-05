use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_51801746: FileFormat = FileFormat {
    id: 51_801_746,
    source_type: SourceType::Wikidata,
    name: "Stationery for Mac OS X",
    extensions: &["doc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
