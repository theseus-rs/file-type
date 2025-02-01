use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_21041556: FileFormat = FileFormat {
    id: 21_041_556,
    puid: "wikidata/21041556",
    name: "Music Editor format",
    extensions: &["med"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
