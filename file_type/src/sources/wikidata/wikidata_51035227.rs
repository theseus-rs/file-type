use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_51035227: FileFormat = FileFormat {
    id: 51_035_227,
    source_type: SourceType::Wikidata,
    name: "Paradox Database Table, version 4",
    extensions: &["db"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
