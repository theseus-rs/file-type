use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967222: FileFormat = FileFormat {
    id: 27_967_222,
    source_type: SourceType::Wikidata,
    name: "Soundtrakker 128 module",
    extensions: &["128"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
