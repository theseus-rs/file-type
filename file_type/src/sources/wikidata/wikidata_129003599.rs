use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_129003599: FileFormat = FileFormat {
    id: 129_003_599,
    source_type: SourceType::Wikidata,
    name: "eC source code file",
    extensions: &["ec"],
    media_types: &["text/x-echdr", "text/x-ecsrc"],
    internal_signatures: &[],
    related_formats: &[],
};
