use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113557073: FileFormat = FileFormat {
    id: 113_557_073,
    source_type: SourceType::Wikidata,
    name: "CloneCD Image",
    extensions: &["img"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
