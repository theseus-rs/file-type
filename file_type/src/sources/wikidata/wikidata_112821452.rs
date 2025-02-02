use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_112821452: FileFormat = FileFormat {
    id: 112_821_452,
    source_type: SourceType::Wikidata,
    name: "Pro/ENGINEER rendering data file",
    extensions: &["slp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
