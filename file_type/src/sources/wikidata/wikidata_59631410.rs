use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_59631410: FileFormat = FileFormat {
    id: 59_631_410,
    puid: "wikidata/59631410",
    name: "Navisworks Document",
    extensions: &["nwc", "nwd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
