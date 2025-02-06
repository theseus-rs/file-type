use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860347: FileFormat = FileFormat {
    id: 105_860_347,
    source_type: SourceType::Wikidata,
    name: "R documentation (with rem)",
    extensions: &["rd"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
