use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111317361: FileFormat = FileFormat {
    id: 111_317_361,
    source_type: SourceType::Wikidata,
    name: "MAUD sample format",
    extensions: &["maud"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
