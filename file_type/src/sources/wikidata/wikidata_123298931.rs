use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_123298931: FileFormat = FileFormat {
    id: 123_298_931,
    source_type: SourceType::Wikidata,
    name: "Retrospect RRR File",
    extensions: &["rrr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
