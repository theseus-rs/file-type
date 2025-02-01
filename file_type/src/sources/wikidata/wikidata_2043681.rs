use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_2043681: FileFormat = FileFormat {
    id: 2_043_681,
    puid: "wikidata/2043681",
    name: "PAK",
    extensions: &["pak"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
