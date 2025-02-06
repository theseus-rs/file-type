use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_120763430: FileFormat = FileFormat {
    id: 120_763_430,
    source_type: SourceType::Wikidata,
    name: "Topo USA 2.0 File",
    extensions: &["tp2"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
