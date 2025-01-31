use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27866113: FileFormat = FileFormat {
    id: 27_866_113,
    puid: "wikidata/27866113",
    name: "Adobe Illustrator Artwork, version 4.0",
    extensions: &["ai"],
    media_types: &["application/postscript"],
    internal_signatures: &[],
    related_formats: &[],
};
