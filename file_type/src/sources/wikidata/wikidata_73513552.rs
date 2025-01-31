use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_73513552: FileFormat = FileFormat {
    id: 73_513_552,
    puid: "wikidata/73513552",
    name: "Puppy Linux DotPup installer package",
    extensions: &["pup"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
