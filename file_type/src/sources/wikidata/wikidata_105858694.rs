use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858694: FileFormat = FileFormat {
    id: 105_858_694,
    source_type: SourceType::Wikidata,
    name: "Skyscraper simulator Building script (with rem)",
    extensions: &["bld"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
