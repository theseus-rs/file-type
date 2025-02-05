use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27923713: FileFormat = FileFormat {
    id: 27_923_713,
    source_type: SourceType::Wikidata,
    name: "DTED Level 1 Terrain Elevation Data File",
    extensions: &["dt1"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
