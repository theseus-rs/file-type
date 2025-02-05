use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_51035340: FileFormat = FileFormat {
    id: 51_035_340,
    source_type: SourceType::Wikidata,
    name: "Paradox Database Table, version 5",
    extensions: &["db"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
