use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862509: FileFormat = FileFormat {
    id: 105_862_509,
    source_type: SourceType::Wikidata,
    name: "Max Patch",
    extensions: &["maxpat"],
    media_types: &["text/json"],
    signatures: &[],
    related_formats: &[],
};
