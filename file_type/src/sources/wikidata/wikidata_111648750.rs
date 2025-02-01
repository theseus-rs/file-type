use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111648750: FileFormat = FileFormat {
    id: 111_648_750,
    puid: "wikidata/111648750",
    name: "Easy Prints file",
    extensions: &["php"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
