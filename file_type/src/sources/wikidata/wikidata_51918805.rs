use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_51918805: FileFormat = FileFormat {
    id: 51_918_805,
    source_type: SourceType::Wikidata,
    name: "XYWrite Document, version 3",
    extensions: &["xy3"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};
