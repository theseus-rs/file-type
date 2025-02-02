use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_118976922: FileFormat = FileFormat {
    id: 118_976_922,
    source_type: SourceType::Wikidata,
    name: "FreeHand Template 11",
    extensions: &["ft11"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
