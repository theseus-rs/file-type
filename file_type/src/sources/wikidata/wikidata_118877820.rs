use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_118877820: FileFormat = FileFormat {
    id: 118_877_820,
    source_type: SourceType::Wikidata,
    name: "Open Scripting Architecture binary script",
    extensions: &["scpt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
