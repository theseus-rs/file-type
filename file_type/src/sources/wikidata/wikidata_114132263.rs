use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_114132263: FileFormat = FileFormat {
    id: 114_132_263,
    source_type: SourceType::Wikidata,
    name: "Chem3D Cartesian Coordinates 1",
    extensions: &["cc1"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
