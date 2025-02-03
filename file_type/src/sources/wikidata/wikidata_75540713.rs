use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_75540713: FileFormat = FileFormat {
    id: 75_540_713,
    source_type: SourceType::Wikidata,
    name: "Ulead PhotoImpact Object",
    extensions: &["ufo"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
