use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_120785583: FileFormat = FileFormat {
    id: 120_785_583,
    source_type: SourceType::Wikidata,
    name: "BusinessCards format",
    extensions: &["biz"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
