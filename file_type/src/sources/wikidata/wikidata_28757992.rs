use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28757992: FileFormat = FileFormat {
    id: 28_757_992,
    source_type: SourceType::Wikidata,
    name: "ISZ",
    extensions: &["isz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
