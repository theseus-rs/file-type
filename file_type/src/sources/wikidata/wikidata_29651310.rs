use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29651310: FileFormat = FileFormat {
    id: 29_651_310,
    puid: "wikidata/29651310",
    name: "Pixie",
    extensions: &["pxi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
