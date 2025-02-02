use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_7493698: FileFormat = FileFormat {
    id: 7_493_698,
    source_type: SourceType::Wikidata,
    name: "Shell Scrap Object File",
    extensions: &["shs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
