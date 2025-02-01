use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130633933: FileFormat = FileFormat {
    id: 130_633_933,
    puid: "wikidata/130633933",
    name: "Ride source code file",
    extensions: &["ride"],
    media_types: &["text/x-ride"],
    internal_signatures: &[],
    related_formats: &[],
};
