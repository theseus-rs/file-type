use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_114093817: FileFormat = FileFormat {
    id: 114_093_817,
    source_type: SourceType::Wikidata,
    name: "Media Hash List",
    extensions: &["mhl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
