use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117448429: FileFormat = FileFormat {
    id: 117_448_429,
    puid: "wikidata/117448429",
    name: "CHAT Transcription Format",
    extensions: &["cha"],
    media_types: &["text/x-chat"],
    internal_signatures: &[],
    related_formats: &[],
};
