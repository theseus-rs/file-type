use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113584320: FileFormat = FileFormat {
    id: 113_584_320,
    puid: "wikidata/113584320",
    name: "Viscosity file",
    extensions: &["vsc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
