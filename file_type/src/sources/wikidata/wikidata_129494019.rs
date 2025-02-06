use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_129494019: FileFormat = FileFormat {
    id: 129_494_019,
    source_type: SourceType::Wikidata,
    name: "GSQL query file",
    extensions: &["gsql"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
