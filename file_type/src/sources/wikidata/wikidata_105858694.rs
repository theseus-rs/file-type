use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858694: FileFormat = FileFormat {
    id: 105_858_694,
    source_type: SourceType::Wikidata,
    name: "Skyscraper simulator Building script (with rem)",
    extensions: &["bld"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
