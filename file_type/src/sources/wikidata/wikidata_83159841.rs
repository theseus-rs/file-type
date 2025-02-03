use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_83159841: FileFormat = FileFormat {
    id: 83_159_841,
    source_type: SourceType::Wikidata,
    name: "CRN",
    extensions: &["crn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
