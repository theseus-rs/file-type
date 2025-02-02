use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28777687: FileFormat = FileFormat {
    id: 28_777_687,
    source_type: SourceType::Wikidata,
    name: "Mono",
    extensions: &["mono"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
