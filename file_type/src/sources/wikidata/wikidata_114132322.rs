use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_114132322: FileFormat = FileFormat {
    id: 114_132_322,
    puid: "wikidata/114132322",
    name: "Chem3D Cartesian Coordinates 2",
    extensions: &["cc2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
