use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105865951: FileFormat = FileFormat {
    id: 105_865_951,
    puid: "wikidata/105865951",
    name: "PALASM Design Description",
    extensions: &["pds"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
