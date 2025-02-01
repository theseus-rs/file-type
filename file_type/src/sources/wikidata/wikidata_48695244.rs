use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_48695244: FileFormat = FileFormat {
    id: 48_695_244,
    puid: "wikidata/48695244",
    name: "DEC Data Exchange File",
    extensions: &["dx"],
    media_types: &["application/dec-dx"],
    internal_signatures: &[],
    related_formats: &[],
};
