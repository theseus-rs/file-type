use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125847329: FileFormat = FileFormat {
    id: 125_847_329,
    puid: "wikidata/125847329",
    name: "D source code file",
    extensions: &["D"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
