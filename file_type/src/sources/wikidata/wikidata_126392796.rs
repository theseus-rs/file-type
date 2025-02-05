use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_126392796: FileFormat = FileFormat {
    id: 126_392_796,
    source_type: SourceType::Wikidata,
    name: "Fotoman RAW",
    extensions: &["pxn"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
