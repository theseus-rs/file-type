use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111316769: FileFormat = FileFormat {
    id: 111_316_769,
    source_type: SourceType::Wikidata,
    name: "Impulse Tracker instrument",
    extensions: &["iti"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
