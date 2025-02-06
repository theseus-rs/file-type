use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_123299060: FileFormat = FileFormat {
    id: 123_299_060,
    source_type: SourceType::Wikidata,
    name: "Ancestry.com Family Tree Database",
    extensions: &["aft"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
