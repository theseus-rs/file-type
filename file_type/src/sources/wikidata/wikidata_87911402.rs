use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_87911402: FileFormat = FileFormat {
    id: 87_911_402,
    source_type: SourceType::Wikidata,
    name: "Avery DesignPro Document 5",
    extensions: &["zdl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
