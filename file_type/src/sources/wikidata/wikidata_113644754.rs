use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113644754: FileFormat = FileFormat {
    id: 113_644_754,
    source_type: SourceType::Wikidata,
    name: "Hayes JT FAX",
    extensions: &["001"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
