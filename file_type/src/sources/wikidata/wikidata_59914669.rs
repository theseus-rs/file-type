use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_59914669: FileFormat = FileFormat {
    id: 59_914_669,
    puid: "wikidata/59914669",
    name: "Steel Detailing Neutral Format",
    extensions: &["sdn"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
