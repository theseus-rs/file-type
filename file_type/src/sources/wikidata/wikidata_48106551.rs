use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_48106551: FileFormat = FileFormat {
    id: 48_106_551,
    puid: "wikidata/48106551",
    name: "DataFlex Query Tag Name",
    extensions: &["tag"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
