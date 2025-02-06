use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27966897: FileFormat = FileFormat {
    id: 27_966_897,
    source_type: SourceType::Wikidata,
    name: "Hudson Entertainment System",
    extensions: &["hes"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
