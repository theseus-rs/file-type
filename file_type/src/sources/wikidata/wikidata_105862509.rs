use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105862509: FileFormat = FileFormat {
    id: 105_862_509,
    source_type: SourceType::Wikidata,
    name: "Max Patch",
    extensions: &["maxpat"],
    media_types: &["text/json"],
    internal_signatures: &[],
    related_formats: &[],
};
