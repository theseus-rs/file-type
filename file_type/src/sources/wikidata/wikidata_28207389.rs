use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28207389: FileFormat = FileFormat {
    id: 28_207_389,
    source_type: SourceType::Wikidata,
    name: "TIM",
    extensions: &["tim"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
