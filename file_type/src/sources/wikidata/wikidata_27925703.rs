use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27925703: FileFormat = FileFormat {
    id: 27_925_703,
    source_type: SourceType::Wikidata,
    name: "DTED Level 0 Maximum Terrain Elevation Value File",
    extensions: &["max"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
