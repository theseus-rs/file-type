use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_83276106: FileFormat = FileFormat {
    id: 83_276_106,
    puid: "wikidata/83276106",
    name: "Interleaf/Quicksilver file format",
    extensions: &["ildoc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
