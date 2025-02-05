use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27925699: FileFormat = FileFormat {
    id: 27_925_699,
    source_type: SourceType::Wikidata,
    name: "DTED Level 0 Average Terrain Elevation Value File",
    extensions: &["avg"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
