use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110535991: FileFormat = FileFormat {
    id: 110_535_991,
    puid: "wikidata/110535991",
    name: "Movie Magic Screenwriter backup document",
    extensions: &["bk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
