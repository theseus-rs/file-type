use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_94279981: FileFormat = FileFormat {
    id: 94_279_981,
    source_type: SourceType::Wikidata,
    name: "Dragon",
    extensions: &["dgn"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
