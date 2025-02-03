use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_115102946: FileFormat = FileFormat {
    id: 115_102_946,
    source_type: SourceType::Wikidata,
    name: "BFRES file",
    extensions: &["bfres"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
