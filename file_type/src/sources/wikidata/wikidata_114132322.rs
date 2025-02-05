use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_114132322: FileFormat = FileFormat {
    id: 114_132_322,
    source_type: SourceType::Wikidata,
    name: "Chem3D Cartesian Coordinates 2",
    extensions: &["cc2"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
