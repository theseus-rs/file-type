use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_122302400: FileFormat = FileFormat {
    id: 122_302_400,
    source_type: SourceType::Wikidata,
    name: "HLD File",
    extensions: &["hld"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
