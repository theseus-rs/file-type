use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_119819196: FileFormat = FileFormat {
    id: 119_819_196,
    source_type: SourceType::Wikidata,
    name: "Final Draft AV Script",
    extensions: &["av"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
