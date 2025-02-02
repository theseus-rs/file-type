use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_114132322: FileFormat = FileFormat {
    id: 114_132_322,
    source_type: SourceType::Wikidata,
    name: "Chem3D Cartesian Coordinates 2",
    extensions: &["cc2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
