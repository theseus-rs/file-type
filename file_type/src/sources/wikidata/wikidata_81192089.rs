use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_81192089: FileFormat = FileFormat {
    id: 81_192_089,
    source_type: SourceType::Wikidata,
    name: "Infinity Engine Compiled Script",
    extensions: &["bcs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
