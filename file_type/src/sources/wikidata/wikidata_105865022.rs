use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105865022: FileFormat = FileFormat {
    id: 105_865_022,
    puid: "wikidata/105865022",
    name: "PETSCII character graphics",
    extensions: &["psci"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
