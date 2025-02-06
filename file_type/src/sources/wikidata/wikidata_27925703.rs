use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27925703: FileFormat = FileFormat {
    id: 27_925_703,
    source_type: SourceType::Wikidata,
    name: "DTED Level 0 Maximum Terrain Elevation Value File",
    extensions: &["max"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
