use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27966897: FileFormat = FileFormat {
    id: 27_966_897,
    source_type: SourceType::Wikidata,
    name: "Hudson Entertainment System",
    extensions: &["hes"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
