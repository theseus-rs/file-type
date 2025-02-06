use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_67383890: FileFormat = FileFormat {
    id: 67_383_890,
    source_type: SourceType::Wikidata,
    name: "Source Engine Compiled AI Nodegraph",
    extensions: &["ain"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
