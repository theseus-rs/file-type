use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_119819196: FileFormat = FileFormat {
    id: 119_819_196,
    puid: "wikidata/119819196",
    name: "Final Draft AV Script",
    extensions: &["av"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
