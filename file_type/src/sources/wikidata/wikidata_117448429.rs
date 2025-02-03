use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117448429: FileFormat = FileFormat {
    id: 117_448_429,
    source_type: SourceType::Wikidata,
    name: "CHAT Transcription Format",
    extensions: &["cha"],
    media_types: &["text/x-chat"],
    internal_signatures: &[],
    related_formats: &[],
};
