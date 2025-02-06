use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_119973449: FileFormat = FileFormat {
    id: 119_973_449,
    source_type: SourceType::Wikidata,
    name: "WebEasy Web Document",
    extensions: &["web"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
