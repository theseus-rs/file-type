use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_51034969: FileFormat = FileFormat {
    id: 51_034_969,
    source_type: SourceType::Wikidata,
    name: "Paradox Database Table format, version 3",
    extensions: &["db"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
