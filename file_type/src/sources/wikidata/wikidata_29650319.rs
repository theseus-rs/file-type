use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29650319: FileFormat = FileFormat {
    id: 29_650_319,
    source_type: SourceType::Wikidata,
    name: "PIM",
    extensions: &["pim"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
