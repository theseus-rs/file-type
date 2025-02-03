use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_2053: FileFormat = FileFormat {
    id: 2_053,
    source_type: SourceType::Wikidata,
    name: "HTML5",
    extensions: &["htm", "html"],
    media_types: &["text/html"],
    internal_signatures: &[],
    related_formats: &[],
};
