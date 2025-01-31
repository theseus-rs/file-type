use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_114132263: FileFormat = FileFormat {
    id: 114_132_263,
    puid: "wikidata/114132263",
    name: "Chem3D Cartesian Coordinates 1",
    extensions: &["cc1"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
