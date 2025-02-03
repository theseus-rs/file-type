use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111417253: FileFormat = FileFormat {
    id: 111_417_253,
    source_type: SourceType::Wikidata,
    name: "Resource Script format",
    extensions: &["rc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
