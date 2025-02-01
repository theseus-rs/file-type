use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27866118: FileFormat = FileFormat {
    id: 27_866_118,
    puid: "wikidata/27866118",
    name: "Adobe Illustrator Artwork, version 5.x Japenese Edition",
    extensions: &["ai"],
    media_types: &["application/postscript"],
    internal_signatures: &[],
    related_formats: &[],
};
