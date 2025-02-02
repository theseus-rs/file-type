use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_124845089: FileFormat = FileFormat {
    id: 124_845_089,
    source_type: SourceType::Wikidata,
    name: "mh",
    extensions: &["mh"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
