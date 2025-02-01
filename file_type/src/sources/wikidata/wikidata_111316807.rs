use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111316807: FileFormat = FileFormat {
    id: 111_316_807,
    puid: "wikidata/111316807",
    name: "Kurzweil K2500 file",
    extensions: &["k25"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
