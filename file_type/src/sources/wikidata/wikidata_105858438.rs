use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858438: FileFormat = FileFormat {
    id: 105_858_438,
    source_type: SourceType::Wikidata,
    name: "ESU electronic sounds",
    extensions: &["esu"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
