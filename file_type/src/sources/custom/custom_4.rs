use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const CUSTOM_4: FileFormat = FileFormat {
    id: 4,
    puid: "custom/4",
    name: "JSON Lines",
    extensions: &["jsonl"],
    media_types: &["application/jsonl"],
    internal_signatures: &[],
    related_formats: &[],
};
