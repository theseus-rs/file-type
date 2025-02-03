use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858335: FileFormat = FileFormat {
    id: 105_858_335,
    source_type: SourceType::Wikidata,
    name: "Adobe Edge Project",
    extensions: &["edge"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
