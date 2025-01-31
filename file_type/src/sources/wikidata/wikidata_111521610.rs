use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111521610: FileFormat = FileFormat {
    id: 111_521_610,
    puid: "wikidata/111521610",
    name: "Packed-Ice True Colour Sprites",
    extensions: &["trs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
