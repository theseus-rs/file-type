use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857499: FileFormat = FileFormat {
    id: 105_857_499,
    puid: "wikidata/105857499",
    name: "Shaper LUT (with rem)",
    extensions: &["3dl"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
