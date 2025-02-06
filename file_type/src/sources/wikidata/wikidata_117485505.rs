use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117485505: FileFormat = FileFormat {
    id: 117_485_505,
    source_type: SourceType::Wikidata,
    name: "MacCaption Project",
    extensions: &["cca"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
