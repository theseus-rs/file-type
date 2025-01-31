use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_115102946: FileFormat = FileFormat {
    id: 115_102_946,
    puid: "wikidata/115102946",
    name: "BFRES file",
    extensions: &["bfres"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
