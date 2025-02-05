use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27925700: FileFormat = FileFormat {
    id: 27_925_700,
    source_type: SourceType::Wikidata,
    name: "DTED Level 0 Minimum Terrain Elevation Value File",
    extensions: &["min"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
