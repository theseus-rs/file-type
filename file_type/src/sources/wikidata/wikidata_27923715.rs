use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27923715: FileFormat = FileFormat {
    id: 27_923_715,
    source_type: SourceType::Wikidata,
    name: "DTED Level 2 Terrain Elevation Data File",
    extensions: &["dt2"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
