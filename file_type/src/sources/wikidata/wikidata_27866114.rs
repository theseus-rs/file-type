use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27866114: FileFormat = FileFormat {
    id: 27_866_114,
    puid: "wikidata/27866114",
    name: "Adobe Illustrator Artwork, version 5.0/5.5",
    extensions: &["ai"],
    media_types: &["application/postscript"],
    internal_signatures: &[],
    related_formats: &[],
};
