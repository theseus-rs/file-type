use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27923712: FileFormat = FileFormat {
    id: 27_923_712,
    source_type: SourceType::Wikidata,
    name: "DTED Level 0 Terrain Elevation Data File",
    extensions: &["dt0"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
