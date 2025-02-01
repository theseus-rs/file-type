use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_63036182: FileFormat = FileFormat {
    id: 63_036_182,
    puid: "wikidata/63036182",
    name: "8-bit ASCII Text",
    extensions: &["asc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
