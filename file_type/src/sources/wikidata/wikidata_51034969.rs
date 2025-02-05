use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_51034969: FileFormat = FileFormat {
    id: 51_034_969,
    source_type: SourceType::Wikidata,
    name: "Paradox Database Table format, version 3",
    extensions: &["db"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
