use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_123298931: FileFormat = FileFormat {
    id: 123_298_931,
    source_type: SourceType::Wikidata,
    name: "Retrospect RRR File",
    extensions: &["rrr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
