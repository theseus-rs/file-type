use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117035683: FileFormat = FileFormat {
    id: 117_035_683,
    source_type: SourceType::Wikidata,
    name: "FloorPlan file",
    extensions: &["bmf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
