use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27866092: FileFormat = FileFormat {
    id: 27_866_092,
    puid: "wikidata/27866092",
    name: "Adobe Illustrator Artwork, version 1.0/1.1",
    extensions: &["ai"],
    media_types: &["application/postscript"],
    internal_signatures: &[],
    related_formats: &[],
};
