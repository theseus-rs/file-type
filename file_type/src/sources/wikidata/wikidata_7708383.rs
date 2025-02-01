use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_7708383: FileFormat = FileFormat {
    id: 7_708_383,
    puid: "wikidata/7708383",
    name: "textClipping",
    extensions: &["textClipping"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
