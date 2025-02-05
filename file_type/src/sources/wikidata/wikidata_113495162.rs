use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113495162: FileFormat = FileFormat {
    id: 113_495_162,
    source_type: SourceType::Wikidata,
    name: "Calc602 Project File 1.0",
    extensions: &["pc6"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
