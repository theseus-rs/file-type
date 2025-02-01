use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_34735750: FileFormat = FileFormat {
    id: 34_735_750,
    puid: "wikidata/34735750",
    name: "SimTower saved game",
    extensions: &["tdt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
