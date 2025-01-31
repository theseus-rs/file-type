use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111316933: FileFormat = FileFormat {
    id: 111_316_933,
    puid: "wikidata/111316933",
    name: "Kurzweil K2600 file",
    extensions: &["k26"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
