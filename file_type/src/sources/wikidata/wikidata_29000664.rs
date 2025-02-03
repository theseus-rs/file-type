use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29000664: FileFormat = FileFormat {
    id: 29_000_664,
    source_type: SourceType::Wikidata,
    name: "Processed Volume",
    extensions: &["pvl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
