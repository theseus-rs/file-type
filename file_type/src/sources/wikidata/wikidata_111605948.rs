use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111605948: FileFormat = FileFormat {
    id: 111_605_948,
    puid: "wikidata/111605948",
    name: "Adobe InDesign Document, version CC 2020",
    extensions: &["ind", "indd", "indt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
