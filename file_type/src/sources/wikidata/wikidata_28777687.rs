use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28777687: FileFormat = FileFormat {
    id: 28_777_687,
    source_type: SourceType::Wikidata,
    name: "Mono",
    extensions: &["mono"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
