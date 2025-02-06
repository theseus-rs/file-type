use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125040222: FileFormat = FileFormat {
    id: 125_040_222,
    source_type: SourceType::Wikidata,
    name: "Syntorial file",
    extensions: &["syn"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
