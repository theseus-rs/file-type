use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117448429: FileFormat = FileFormat {
    id: 117_448_429,
    source_type: SourceType::Wikidata,
    name: "CHAT Transcription Format",
    extensions: &["cha"],
    media_types: &["text/x-chat"],
    signatures: &[],
    related_formats: &[],
};
