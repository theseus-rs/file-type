use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27866112: FileFormat = FileFormat {
    id: 27_866_112,
    puid: "wikidata/27866112",
    name: "Adobe Illustrator Artwork, version 3.0/3.2",
    extensions: &["ai"],
    media_types: &["application/postscript"],
    internal_signatures: &[],
    related_formats: &[],
};
