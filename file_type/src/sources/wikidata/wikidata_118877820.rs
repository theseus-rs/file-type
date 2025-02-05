use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_118877820: FileFormat = FileFormat {
    id: 118_877_820,
    source_type: SourceType::Wikidata,
    name: "Open Scripting Architecture binary script",
    extensions: &["scpt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
