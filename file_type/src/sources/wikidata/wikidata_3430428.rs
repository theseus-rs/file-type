use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_3430428: FileFormat = FileFormat {
    id: 3_430_428,
    source_type: SourceType::Wikidata,
    name: "Rich Text Format Directory",
    extensions: &["rtfd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
