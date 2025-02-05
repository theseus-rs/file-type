use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_94279981: FileFormat = FileFormat {
    id: 94_279_981,
    source_type: SourceType::Wikidata,
    name: "Dragon",
    extensions: &["dgn"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
