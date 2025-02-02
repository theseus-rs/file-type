use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_25103897: FileFormat = FileFormat {
    id: 25_103_897,
    source_type: SourceType::Wikidata,
    name: "Dynamic Text Document",
    extensions: &["dtxt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
