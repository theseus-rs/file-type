use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_126392796: FileFormat = FileFormat {
    id: 126_392_796,
    source_type: SourceType::Wikidata,
    name: "Fotoman RAW",
    extensions: &["pxn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
