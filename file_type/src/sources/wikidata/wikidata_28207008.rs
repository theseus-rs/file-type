use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28207008: FileFormat = FileFormat {
    id: 28_207_008,
    source_type: SourceType::Wikidata,
    name: "Picture Publisher 4",
    extensions: &["pp4"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
