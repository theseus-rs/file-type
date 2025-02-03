use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28207008: FileFormat = FileFormat {
    id: 28_207_008,
    source_type: SourceType::Wikidata,
    name: "Picture Publisher 4",
    extensions: &["pp4"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
