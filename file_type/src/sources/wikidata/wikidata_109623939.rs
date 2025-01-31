use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_109623939: FileFormat = FileFormat {
    id: 109_623_939,
    puid: "wikidata/109623939",
    name: "WritePlus",
    extensions: &["stt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
