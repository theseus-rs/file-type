use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_119819196: FileFormat = FileFormat {
    id: 119_819_196,
    source_type: SourceType::Wikidata,
    name: "Final Draft AV Script",
    extensions: &["av"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
