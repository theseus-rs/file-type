use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_34735519: FileFormat = FileFormat {
    id: 34_735_519,
    puid: "wikidata/34735519",
    name: "Signum font",
    extensions: &["e24", "l30", "p24", "p9"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
