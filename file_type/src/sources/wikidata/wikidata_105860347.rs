use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860347: FileFormat = FileFormat {
    id: 105_860_347,
    puid: "wikidata/105860347",
    name: "R documentation (with rem)",
    extensions: &["rd"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
