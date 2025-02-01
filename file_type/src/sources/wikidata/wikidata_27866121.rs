use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27866121: FileFormat = FileFormat {
    id: 27_866_121,
    puid: "wikidata/27866121",
    name: "Adobe Illustrator Artwork file format, version 7.0",
    extensions: &["ai"],
    media_types: &["application/postscript"],
    internal_signatures: &[],
    related_formats: &[],
};
