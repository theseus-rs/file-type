use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28207152: FileFormat = FileFormat {
    id: 28_207_152,
    source_type: SourceType::Wikidata,
    name: "PTG",
    extensions: &["ptg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
