use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_131418899: FileFormat = FileFormat {
    id: 131_418_899,
    source_type: SourceType::Wikidata,
    name: "Web IDL file format",
    extensions: &["webidl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
