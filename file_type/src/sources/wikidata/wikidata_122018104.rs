use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_122018104: FileFormat = FileFormat {
    id: 122_018_104,
    puid: "wikidata/122018104",
    name: "Stationery Brochures and More Document",
    extensions: &["dtp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
