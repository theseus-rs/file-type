use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27979150: FileFormat = FileFormat {
    id: 27_979_150,
    puid: "wikidata/27979150",
    name: "AN2",
    extensions: &["an2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
