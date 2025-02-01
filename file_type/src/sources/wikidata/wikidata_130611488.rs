use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130611488: FileFormat = FileFormat {
    id: 130_611_488,
    puid: "wikidata/130611488",
    name: "Red language file format",
    extensions: &["red", "red", "reds", "reds"],
    media_types: &[
        "text/x-red",
        "text/x-red",
        "text/x-red-system",
        "text/x-red-system",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
