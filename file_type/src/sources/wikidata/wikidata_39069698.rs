use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_39069698: FileFormat = FileFormat {
    id: 39_069_698,
    source_type: SourceType::Wikidata,
    name: "Ion",
    extensions: &["ion"],
    media_types: &["application/ion"],
    internal_signatures: &[],
    related_formats: &[],
};
