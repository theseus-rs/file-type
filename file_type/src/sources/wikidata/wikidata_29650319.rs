use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29650319: FileFormat = FileFormat {
    id: 29_650_319,
    source_type: SourceType::Wikidata,
    name: "PIM",
    extensions: &["pim"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
