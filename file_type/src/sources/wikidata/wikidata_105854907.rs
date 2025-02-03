use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854907: FileFormat = FileFormat {
    id: 105_854_907,
    source_type: SourceType::Wikidata,
    name: "Artificial Intelligence Markup Language",
    extensions: &["aiml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
