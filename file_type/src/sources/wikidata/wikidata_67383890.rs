use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_67383890: FileFormat = FileFormat {
    id: 67_383_890,
    source_type: SourceType::Wikidata,
    name: "Source Engine Compiled AI Nodegraph",
    extensions: &["ain"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
