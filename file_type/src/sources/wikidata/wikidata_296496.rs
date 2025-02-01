use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_296496: FileFormat = FileFormat {
    id: 296_496,
    puid: "wikidata/296496",
    name: "ARC",
    extensions: &["arc", "ark", "sue"],
    media_types: &[
        "application/x-arc",
        "application/x-arc",
        "application/x-arc",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
