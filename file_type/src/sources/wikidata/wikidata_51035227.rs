use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_51035227: FileFormat = FileFormat {
    id: 51_035_227,
    source_type: SourceType::Wikidata,
    name: "Paradox Database Table, version 4",
    extensions: &["db"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
