use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206498: FileFormat = FileFormat {
    id: 28_206_498,
    source_type: SourceType::Wikidata,
    name: "Age of Empires Graphics File",
    extensions: &["slp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
