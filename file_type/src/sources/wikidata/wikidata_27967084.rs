use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967084: FileFormat = FileFormat {
    id: 27_967_084,
    source_type: SourceType::Wikidata,
    name: "Game Music Creator",
    extensions: &["gmc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
