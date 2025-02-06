use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117035683: FileFormat = FileFormat {
    id: 117_035_683,
    source_type: SourceType::Wikidata,
    name: "FloorPlan file",
    extensions: &["bmf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
