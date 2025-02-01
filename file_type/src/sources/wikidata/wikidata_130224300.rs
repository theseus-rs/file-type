use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130224300: FileFormat = FileFormat {
    id: 130_224_300,
    puid: "wikidata/130224300",
    name: "Lean 4 file format",
    extensions: &["lean"],
    media_types: &["text/x-lean4"],
    internal_signatures: &[],
    related_formats: &[],
};
